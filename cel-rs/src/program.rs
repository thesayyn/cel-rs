use crate::context::Context;
use crate::eval::{Bag, Eval};
use crate::parser::cel::ExpressionParser;
use crate::parser::Expression;
use crate::Value;
use std::fmt;
use std::result::Result;

pub struct Program {
    expr: Expression,
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
            Ok(expr) => Ok(Program { expr }),
            Err(e) => Err(ParseError {
                message: format!("{}", e),
            }),
        }
    }

    pub fn execute(self, context: &mut Context) -> bool {
        self.eval(context).unpack().into()
    }

    pub fn eval(self, context: &mut Context) -> Value {
        let e = Eval::default();
        e.eval(self.expr, context).unpack()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        program,
        value::{Function, Overload},
        Value,
    };

    macro_rules! string {
        ($q:literal) => {
            crate::Value::String(std::rc::Rc::new($q.into()))
        };
    }
    macro_rules! eval_program {
        ($expr:literal) => {{
            eval_program!($expr, &mut crate::context::Context::default())
        }};
        ($expr:literal, $ctx:expr) => {{
            let program = crate::program::Program::new($expr);
            assert!(
                program.is_ok(),
                "failed to create the program {:?}",
                program.err()
            );
            let program = program.unwrap();
            program.eval($ctx)
        }};
    }

    #[test]
    fn basic_test() {
        assert_eq!(eval_program!(r#"r"""#), string!(""));
    }

    fn calc_string_string(args: Vec<Value>) -> Value {
        println!("{:?}", args);
        let mut args = args.into_iter();
        let a = args.next().unwrap();
        let b = args.next().unwrap();
        a + b
    }

    #[test]
    fn fn_test() {
        let func = Function {
            name: "calc",
            overloads: &[Overload {
                key: "calc_string",
                func: calc_string_string,
            }],
        };
        let mut ctx = program::Context::default()
            .add_variable("a", Value::Int(10))
            .add_variable("b", Value::Int(10))
            .add_variable("calc", crate::Value::Function(func.into()));
        assert_eq!(eval_program!(r#"b.calc(a)"#, &mut ctx), Value::Int(20));
    }
}
