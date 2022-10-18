use std::time::Instant;

use crate::episode::episode;
use crate::model::TrainableModel;
use oppai_field::field::Field;
use oppai_field::player::Player;
use rand::Rng;

const ITERATIONS_NUMBER: u32 = 10000;

pub fn self_play<E, M, R>(field: &Field, player: Player, model: &M, rng: &mut R) -> Result<(), E>
where
  M: TrainableModel<E = E> + Clone,
  R: Rng,
{
  let start_time = Instant::now();

  for i in 0..ITERATIONS_NUMBER {
    log::info!("Episode {}, elapsed {:?}", i, start_time.elapsed());

    let mut cur_field = field.clone();
    let (inputs, policies, values) = episode(&mut cur_field, player, model, rng)?;
    model.train(inputs, policies, values)?;
    model.save()?;
  }

  Ok(())
}
