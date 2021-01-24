use oppai_minimax::minimax::MinimaxConfig;
use oppai_uct::uct::UctConfig;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Copy, PartialEq, Debug, EnumString, EnumVariantNames)]
pub enum Solver {
  Uct,
  Minimax,
  Heuristic,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
  pub uct: UctConfig,
  pub minimax: MinimaxConfig,
  pub time_gap: u32,
  pub solver: Solver,
}