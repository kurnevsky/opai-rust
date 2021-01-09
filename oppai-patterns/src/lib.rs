#[macro_use]
extern crate log;

pub mod dfa;
pub mod patterns;
pub mod rotate;
pub mod spiral;

#[cfg(test)]
mod patterns_test;
