//! Helpers to generate code from an IR instance and fully specified decisions.
mod cfg;
mod dimension;
mod function;
mod namer;

pub use self::cfg::Cfg;
pub use self::dimension::{Dimension, InductionLevel, InductionVar};
pub use self::function::*;
pub use self::namer::{NameMap, Namer, Value};

// TODO(cleanup): refactor function
// - extend instructions with additional information: vector factor, flag, instantiated
// dims TODO(cleanup): refactor namer
