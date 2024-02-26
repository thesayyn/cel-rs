use parser::Expression;
use parser::parser::ExpressionParser;
use std::fmt;
use std::result::Result;
use crate::context::Context;
use crate::eval::{Bag, Eval};
use crate::Value;

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

    pub fn execute(self, context: Context) -> bool {
        self.eval(context).unpack().into()
    }

    pub fn eval(self, context: Context) -> Value {
        let e = Eval::new(context);
        e.eval(self.expr).unpack()
    }
}



#[cfg(test)]
pub mod tests {
    macro_rules! string {
        ($q:literal) => {
            crate::Value::String(std::rc::Rc::new($q.into()))
        };
    }
    macro_rules! eval_program {
        ($expr:literal) => ({
            let program = crate::program::Program::new($expr);
            assert!(program.is_ok(), "failed to create the program {:?}", program.err());
            let program = program.unwrap();
            program.eval(crate::context::Context::default())
        });
    }

    #[test]
    fn basic_test() {
        assert_eq!(eval_program!(r#"r"""#), string!(""));
    }
}