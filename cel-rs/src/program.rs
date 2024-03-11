use crate::parser::Expression;
use crate::parser::cel::ExpressionParser;
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
    use std::rc::Rc;

    use crate::{program, value::{FnValue, Overload}, Value};

    macro_rules! string {
        ($q:literal) => {
            crate::Value::String(std::rc::Rc::new($q.into()))
        };
    }
    macro_rules! eval_program {
        ($expr:literal) => ({
           eval_program!($expr, &mut crate::context::Context::default())
        });
        ($expr:literal, $ctx:expr) => ({
            let program = crate::program::Program::new($expr);
            assert!(program.is_ok(), "failed to create the program {:?}", program.err());
            let program = program.unwrap();
            program.eval($ctx)
        });
    }

    #[test]
    fn basic_test() {
        assert_eq!(eval_program!(r#"r"""#), string!(""));
    }

    fn calc_string_string(lhs: &Value, rhs: &Value) -> Value {
        Value::Null
    }

    #[test]
    fn fn_test() {
        let func = FnValue {
            name: "calc",
            overloads: &[
                Overload {
                    key: "calc_string",
                    func: calc_string_string
                }
            ],
        };
        let mut ctx = program::Context::default()
        .add_variable("a", Value::String(Rc::new("".into())))
        .add_variable("b", Value::Int(0))
        .add_variable("calc", crate::Value::Function(Rc::new(func)));
        assert_eq!(eval_program!(r#"b.calc(a)"#, &mut ctx), string!(""));
    }
}
