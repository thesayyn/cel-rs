mod context;
mod value;
mod program;
mod eval;
mod parser;

// public api
pub use crate::program::Program;
pub use crate::context::Context;
pub use value::value::{Val, Value};