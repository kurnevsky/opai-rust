use num_cpus;
use types::{CoordSum, Depth};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UcbType {
  Ucb1,
  Ucb1Tuned
}

static UCT_RADIUS: CoordSum = 3;

static UCB_TYPE: UcbType = UcbType::Ucb1Tuned;

static UCT_DRAW_WEIGHT: f32 = 0.4;

static UCTK: f32 = 1.0;

static UCT_WHEN_CREATE_CHILDREN: usize = 2;

static UCT_DEPTH: Depth = 8;

static mut THREADS_COUNT: usize = 4;

static DYNAMIC_KOMI: bool = false;

static UCT_RED: f32 = 0.45;

static UCT_GREEN: f32 = 0.5;

static UCT_KOMI_INTERVAL: usize = 10;

static UCT_KOMI_MIN_ITERATIONS: usize = 1000;

pub fn init() {
  unsafe {
    THREADS_COUNT = num_cpus::get();
  }
}

#[inline]
pub fn uct_radius() -> CoordSum {
  UCT_RADIUS
}

#[inline]
pub fn ucb_type() -> UcbType {
  UCB_TYPE
}

#[inline]
pub fn uct_draw_weight() -> f32 {
  UCT_DRAW_WEIGHT
}

#[inline]
pub fn uctk() -> f32 {
  UCTK
}

#[inline]
pub fn uct_when_create_children() -> usize {
  UCT_WHEN_CREATE_CHILDREN
}

#[inline]
pub fn uct_depth() -> Depth {
  UCT_DEPTH
}

#[inline]
pub fn threads_count() -> usize {
  unsafe { THREADS_COUNT }
}

#[inline]
pub fn dynamic_komi() -> bool {
  DYNAMIC_KOMI
}

#[inline]
pub fn uct_red() -> f32 {
  UCT_RED
}

#[inline]
pub fn uct_green() -> f32 {
  UCT_GREEN
}

#[inline]
pub fn uct_komi_interval() -> usize {
  UCT_KOMI_INTERVAL
}

#[inline]
pub fn uct_komi_min_iterations() -> usize {
  UCT_KOMI_MIN_ITERATIONS
}
