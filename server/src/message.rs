use oppai_field::player::Player;
use serde::{Deserialize, Serialize};

use crate::ids::*;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct FieldSize {
  pub width: u32,
  pub height: u32,
}

impl FieldSize {
  const MIN_SIZE: u32 = 10;
  const MAX_SIZE: u32 = 50;

  pub fn is_valid(&self) -> bool {
    self.width >= Self::MIN_SIZE
      && self.width <= Self::MAX_SIZE
      && self.height >= Self::MIN_SIZE
      && self.height <= Self::MAX_SIZE
  }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Coordinate {
  pub x: u32,
  pub y: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Move {
  pub coordinate: Coordinate,
  pub player: Player,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenGame {
  pub player_id: PlayerId,
  pub game_id: GameId,
  pub size: FieldSize,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
  pub game_id: GameId,
  pub red_player_id: PlayerId,
  pub black_player_id: PlayerId,
  pub size: FieldSize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AuthProvider {
  Google,
  GitLab,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
#[serde(rename_all_fields = "camelCase")]
pub enum Request {
  GetAuthUrl {
    provider: AuthProvider,
    remember_me: bool,
  },
  Auth {
    code: String,
    state: String,
  },
  #[cfg(feature = "test")]
  AuthTest {
    name: String,
  },
  /// Create a new game in a lobby.
  Create {
    size: FieldSize,
  },
  /// Close an open game.
  Close {
    game_id: GameId,
  },
  /// Join a game from lobby.
  Join {
    game_id: GameId,
  },
  /// Subscribe to game moves.
  Subscribe {
    game_id: GameId,
  },
  /// Subscribe from game moves.
  Unsubscribe {
    game_id: GameId,
  },
  /// Put a point in a game.
  PutPoint {
    game_id: GameId,
    coordinate: Coordinate,
  },
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
#[serde(rename_all_fields = "camelCase")]
pub enum Response {
  /// First message when connection is established.
  Init {
    player_id: Option<PlayerId>,
    open_games: Vec<OpenGame>,
    games: Vec<Game>,
  },
  /// First message after subscription.
  GameInit {
    game_id: GameId,
    moves: Vec<Move>,
  },
  AuthUrl {
    url: String,
  },
  Auth {
    player_id: PlayerId,
    cookie: String,
  },
  /// A new game was created in a lobby.
  Create {
    game_id: GameId,
    player_id: PlayerId,
    size: FieldSize,
  },
  /// An open game was closed.
  Close {
    game_id: GameId,
  },
  /// A new game started.
  Start {
    game_id: GameId,
    red_player_id: PlayerId,
    black_player_id: PlayerId,
  },
  /// A point in a game was put.
  PutPoint {
    game_id: GameId,
    coordinate: Coordinate,
    player: Player,
  },
}