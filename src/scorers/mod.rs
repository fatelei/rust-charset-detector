/// Encoding detection scorers
///
/// This module contains detection and scoring logic for various encoding families

pub mod utf8;
pub mod cjk;

pub use utf8::*;
pub use cjk::*;
