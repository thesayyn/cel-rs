use parser::{Expression};
use parser::parser::ExpressionParser;

use std::fmt;

use std::result::Result;
use crate::context::Context;
use crate::value::Value;

pub struct Program {
    expr: Expression
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Program {
    pub fn new(source: &str) -> Result<Program, ParseError> {
        match ExpressionParser::new().parse(source) {
            Ok(expr) => Ok(Program {expr}),
            Err(e) => Err(ParseError{message: format!("{}", e)}),
        }
    }

    pub fn execute(&self, context: Context) -> bool {
        match Value::from(&self.expr) {
            Value::Bool(b) => b,
            _ => panic!("this was not supposed to happen!")
        }
    }
}


#[test]
fn basic_test() {
    let true_cases = [
        "6 == 6", 
        "6 + 12 == 18", 
        "(6 - 12) == -6"
    ];
    for case in true_cases {
        assert!(Program::new(case).expect("failed to compile").execute(Context::default()));
    }
}