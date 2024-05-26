use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod parse;
pub use ast::*;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub cel, 
    "/parser/cel.rs"
);