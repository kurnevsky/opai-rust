use crate::minimax::{Minimax, MinimaxConfig, MinimaxType};
use oppai_field::construct_field::construct_field;
use oppai_field::field::NonZeroPos;
use oppai_field::player::Player;
use oppai_test_images::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

const SEED: u64 = 7;

const MINIMAX_CONFIG_NEGASCOUT: MinimaxConfig = MinimaxConfig {
  threads_count: 1,
  minimax_type: MinimaxType::NegaScout,
  hash_table_size: 10_000,
  rebuild_trajectories: false,
};

const MINIMAX_CONFIG_MTDF: MinimaxConfig = MinimaxConfig {
  threads_count: 1,
  minimax_type: MinimaxType::Mtdf,
  hash_table_size: 10_000,
  rebuild_trajectories: false,
};

macro_rules! minimax_test {
  ($(#[$($attr:meta),+])* $name:ident, $config:ident, $image:ident, $depth:expr) => {
    #[test]
    $(#[$($attr),+])*
    fn $name() {
      env_logger::try_init().ok();
      let mut rng = Xoshiro256PlusPlus::seed_from_u64(SEED);
      let mut field = construct_field(&mut rng, $image.image);
      let minimax = Minimax::new($config);
      let (pos, _) = minimax.minimax(&mut field, Player::Red, $depth, &|| false);
      assert_eq!(pos, NonZeroPos::new(field.to_pos($image.solution.0, $image.solution.1)));
    }
  }
}

minimax_test!(negascout_1, MINIMAX_CONFIG_NEGASCOUT, IMAGE_1, 8);
minimax_test!(negascout_2, MINIMAX_CONFIG_NEGASCOUT, IMAGE_2, 8);
minimax_test!(negascout_3, MINIMAX_CONFIG_NEGASCOUT, IMAGE_3, 8);
minimax_test!(negascout_4, MINIMAX_CONFIG_NEGASCOUT, IMAGE_4, 8);
minimax_test!(negascout_5, MINIMAX_CONFIG_NEGASCOUT, IMAGE_5, 8);
minimax_test!(negascout_6, MINIMAX_CONFIG_NEGASCOUT, IMAGE_6, 8);
minimax_test!(
  #[ignore]
  negascout_7,
  MINIMAX_CONFIG_NEGASCOUT,
  IMAGE_7,
  10
);
minimax_test!(negascout_8, MINIMAX_CONFIG_NEGASCOUT, IMAGE_8, 8);
minimax_test!(
  #[ignore]
  negascout_9,
  MINIMAX_CONFIG_NEGASCOUT,
  IMAGE_9,
  10
);
minimax_test!(negascout_10, MINIMAX_CONFIG_NEGASCOUT, IMAGE_10, 8);
minimax_test!(
  #[ignore]
  negascout_11,
  MINIMAX_CONFIG_NEGASCOUT,
  IMAGE_11,
  12
);
minimax_test!(negascout_12, MINIMAX_CONFIG_NEGASCOUT, IMAGE_12, 8);
minimax_test!(negascout_13, MINIMAX_CONFIG_NEGASCOUT, IMAGE_13, 8);
minimax_test!(negascout_14, MINIMAX_CONFIG_NEGASCOUT, IMAGE_14, 8);
minimax_test!(negascout_15, MINIMAX_CONFIG_NEGASCOUT, IMAGE_15, 8);

minimax_test!(mtdf_1, MINIMAX_CONFIG_MTDF, IMAGE_1, 8);
minimax_test!(mtdf_2, MINIMAX_CONFIG_MTDF, IMAGE_2, 8);
minimax_test!(mtdf_3, MINIMAX_CONFIG_MTDF, IMAGE_3, 8);
minimax_test!(mtdf_4, MINIMAX_CONFIG_MTDF, IMAGE_4, 8);
minimax_test!(mtdf_5, MINIMAX_CONFIG_MTDF, IMAGE_5, 8);
minimax_test!(mtdf_6, MINIMAX_CONFIG_MTDF, IMAGE_6, 8);
minimax_test!(
  #[ignore]
  mtdf_7,
  MINIMAX_CONFIG_MTDF,
  IMAGE_7,
  10
);
minimax_test!(mtdf_8, MINIMAX_CONFIG_MTDF, IMAGE_8, 8);
minimax_test!(
  #[ignore]
  mtdf_9,
  MINIMAX_CONFIG_MTDF,
  IMAGE_9,
  10
);
minimax_test!(mtdf_10, MINIMAX_CONFIG_MTDF, IMAGE_10, 8);
minimax_test!(
  #[ignore]
  mtdf_11,
  MINIMAX_CONFIG_MTDF,
  IMAGE_11,
  12
);
minimax_test!(mtdf_12, MINIMAX_CONFIG_MTDF, IMAGE_12, 8);
minimax_test!(mtdf_13, MINIMAX_CONFIG_MTDF, IMAGE_13, 8);
minimax_test!(mtdf_14, MINIMAX_CONFIG_MTDF, IMAGE_14, 8);
minimax_test!(mtdf_15, MINIMAX_CONFIG_MTDF, IMAGE_15, 8);
