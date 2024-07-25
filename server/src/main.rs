use anyhow::{Error, Result};
use futures::channel::mpsc::{self, Sender};
use futures_util::{select, FutureExt, SinkExt, StreamExt};
use ids::*;
use im::HashSet;
use oppai_field::field::Field;
use rand::{rngs::StdRng, Rng, SeedableRng};
use state::{FieldSize, Game, OpenGame, State};
use std::sync::Arc;
use tokio::{
  net::{TcpListener, TcpStream},
  sync::RwLock,
};
use tokio_tungstenite::tungstenite::Message;
use uuid::Builder;

mod ids;
mod message;
mod state;

async fn init(state: &State, connection_id: ConnectionId, tx: Sender<message::Response>) -> Result<()> {
  // lock connection before inserting so we can be sure we send init message before any update
  let connection = Arc::new(RwLock::new(tx));
  let connection_c = connection.clone();
  let mut connection_c_lock = connection_c.write().await;

  state.connections.pin().insert(connection_id, connection);

  let open_games = state
    .open_games
    .pin()
    .iter()
    .map(|(&game_id, open_game)| message::OpenGame {
      game_id,
      size: message::FieldSize {
        width: open_game.size.width,
        height: open_game.size.height,
      },
    })
    .collect();
  let games = state
    .games
    .pin()
    .iter()
    .map(|(&game_id, game)| message::Game {
      game_id,
      size: message::FieldSize {
        width: game.size.width,
        height: game.size.height,
      },
    })
    .collect();
  let init = message::Response::Init { open_games, games };
  connection_c_lock.send(init).await?;

  Ok(())
}

fn finalize(state: &State, connection_id: ConnectionId, player_id: PlayerId, watching: &HashSet<GameId>) {
  for &game_id in watching {
    state.unsubscribe(connection_id, game_id);
  }

  state.remove_players_connection(player_id, connection_id);
}

async fn create<R: Rng>(rng: &mut R, player_id: PlayerId, state: &State, size: message::FieldSize) {
  let game_id = GameId(Builder::from_random_bytes(rng.gen()).into_uuid());
  let open_game = OpenGame {
    player_id,
    size: FieldSize {
      width: size.width,
      height: size.height,
    },
  };

  // TODO: how many open games per player to allow?
  state.open_games.pin().insert(game_id, open_game);

  state.send_to_all(message::Response::Create { game_id, size }).await;
}

async fn join<R: Rng>(rng: &mut R, player_id: PlayerId, state: &State, game_id: GameId) {
  let open_game = if let Some(open_game) = state.open_games.pin().remove(&game_id) {
    open_game.clone()
  } else {
    // TODO: log
    return;
  };

  let field = Field::new_from_rng(open_game.size.width, open_game.size.height, rng);
  let game = Game {
    red_player_id: open_game.player_id,
    black_player_id: player_id,
    size: open_game.size,
    field: Arc::new(RwLock::new(field)),
  };

  state.games.pin().insert(game_id, game);

  state.send_to_all(message::Response::Start { game_id }).await;
}

fn subscribe(
  connection_id: ConnectionId,
  watching: &mut HashSet<GameId>,
  state: &State,
  game_id: GameId,
) -> Result<()> {
  if watching.contains(&game_id) {
    anyhow::bail!("already watching");
  }

  state.subscribe(connection_id, game_id);

  Ok(())
}

fn unsubscribe(
  connection_id: ConnectionId,
  watching: &mut HashSet<GameId>,
  state: &State,
  game_id: GameId,
) -> Result<()> {
  if !watching.contains(&game_id) {
    anyhow::bail!("not watching");
  }

  state.unsubscribe(connection_id, game_id);

  Ok(())
}

async fn put_point(player_id: PlayerId, state: &State, game_id: GameId, coordinate: message::Coordinate) -> Result<()> {
  let (field, player) = if let Some(game) = state.games.pin().get(&game_id) {
    let player = if let Some(player) = game.color(player_id) {
      player
    } else {
      anyhow::bail!("putting point is not allowed");
    };
    (game.field.clone(), player)
  } else {
    anyhow::bail!("no game to put point");
  };

  let mut field = field.write().await;
  let pos = field.to_pos(coordinate.x, coordinate.y);
  if !field.put_point(pos, player) {
    anyhow::bail!("putting point on a wrong position");
  }
  drop(field);

  state
    .send_to_watchers(game_id, message::Response::PutPoint { game_id, coordinate })
    .await;

  Ok(())
}

async fn accept_connection<R: Rng>(state: Arc<State>, mut rng: R, stream: TcpStream) -> Result<()> {
  let ws_stream = tokio_tungstenite::accept_async(stream).await?;
  let (mut tx_ws, mut rx_ws) = ws_stream.split();

  let connection_id = ConnectionId(Builder::from_random_bytes(rng.gen()).into_uuid());
  let player_id = PlayerId(Builder::from_random_bytes(rng.gen()).into_uuid());
  let mut watching = HashSet::new();

  let (tx, mut rx) = mpsc::channel::<message::Response>(32);

  init(&state, connection_id, tx).await?;

  state.insert_players_connection(player_id, connection_id);

  let future1 = async {
    while let Some(message) = rx.next().await {
      tx_ws.send(Message::Text(serde_json::to_string(&message)?)).await?;
    }

    Ok::<(), Error>(())
  };

  let future2 = async {
    while let Some(message) = rx_ws.next().await {
      if let Message::Text(message) = message? {
        let message: message::Request = serde_json::from_str(message.as_str())?;
        match message {
          message::Request::Create { size } => create(&mut rng, player_id, &state, size).await,
          message::Request::Join { game_id } => join(&mut rng, player_id, &state, game_id).await,
          message::Request::Subscribe { game_id } => subscribe(connection_id, &mut watching, &state, game_id)?,
          message::Request::Unsubscribe { game_id } => unsubscribe(connection_id, &mut watching, &state, game_id)?,
          message::Request::PutPoint { game_id, coordinate } => {
            put_point(player_id, &state, game_id, coordinate).await?
          }
        }
      }
    }

    // TODO: cleanup

    Ok::<(), Error>(())
  };

  let result = select! {
    r = future1.fuse() => r,
    r = future2.fuse() => r,
  };

  finalize(&state, connection_id, player_id, &watching);

  result
}

#[tokio::main]
async fn main() -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  let state = Arc::new(State::default());

  let mut rng = StdRng::from_entropy();

  loop {
    let (stream, _) = listener.accept().await?;
    // todo log the result
    tokio::spawn(accept_connection(state.clone(), StdRng::from_rng(&mut rng)?, stream));
  }
}
