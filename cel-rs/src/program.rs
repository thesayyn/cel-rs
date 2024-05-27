use crate::context::Context;
use crate::eval::Eval;
use crate::parser::cel::ExpressionParser;
use crate::parser::Expression;
use crate::value::value::Val;
use crate::value::{value::Value};

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
        self.eval(context)
            .to_bool()
            .as_bool()
            .unwrap_or(&false)
            .to_owned()
    }

    pub fn eval(self, context: &mut Context) -> Val {
        let e = Eval::default();
        e.eval(self.expr, context)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{program, value::value::Val};

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
    fn test_bool() {
        let mut ctx = program::Context::default();
        ctx.add_variable("a", Val::new_bool(true));
        assert_eq!(eval_program!(r#"a == true"#, &mut ctx), Val::new_bool(true));
        assert_eq!(eval_program!(r#"a == false"#, &mut ctx), Val::new_bool(false));
    }

    #[test]
    fn test_string() {
        assert_eq!(eval_program!(r#"r"""#), Val::new_string(""));
        assert_eq!(eval_program!(r#"r"CEL""#), Val::new_string("CEL"));
    }

    #[test]
    fn test_null() {
        assert_eq!(eval_program!(r#"null"#), Val::new_null());
    }

    #[test]
    fn test_bytes() {
        assert_eq!(eval_program!(r#"b''"#), Val::new_bytes(vec![].into()));
    }


    #[test]
    fn test_double() {
        assert_eq!(eval_program!(r#"2.0"#), Val::new_double(2.0f64));
    }

    #[test]
    fn test_ints() {
        assert_eq!(eval_program!(r#"2"#), Val::new_int(2));
        assert_eq!(eval_program!(r#"2u"#), Val::new_uint(2));
    }

    #[test]
    fn test_ordering() {
        assert_eq!(eval_program!(r#"2 > 2"#), Val::new_bool(false));
        assert_eq!(eval_program!(r#"2 >= 2"#), Val::new_bool(true));
        assert_eq!(eval_program!(r#"3 > 2"#), Val::new_bool(true));
        assert_eq!(eval_program!(r#"3 >= 2"#), Val::new_bool(true));
        assert_eq!(eval_program!(r#"3 < 2"#), Val::new_bool(false));
        assert_eq!(eval_program!(r#"3 == 2"#), Val::new_bool(false));
        assert_eq!(eval_program!(r#"2 == 2"#), Val::new_bool(true));
    }

    #[test]
    fn self_eval_int_hex_negative() {
        let expr = r#"-0x55555555"#;
        let program = crate::Program::new(expr);
        assert!(program.is_ok(), "failed to parse '{}'", expr);
        let program = program.unwrap();
        let mut ctx = crate::Context::default();
        let value = program.eval(&mut ctx);
        let expected_value = crate::Val::new_int(-1431655765);
        assert_eq!(value, expected_value, r#""{:?}" did not match "{:?}""#, value, expected_value);
    }


//     fn calc_string_string(args: Vec<Value>) -> Value {
//         println!("{:?}", args);
//         let mut args = args.into_iter();
//         let a = args.next().unwrap();
//         let b = args.next().unwrap();
//         a + b
//     }

//     #[test]
//     fn fn_test() {
//         let func = Function {
//             name: "calc",
//             overloads: &[Overload {
//                 key: "calc_string",
//                 func: calc_string_string,
//             }],
//         };
//         let mut ctx = program::Context::default()
//             .add_variable("a", Value::Int(10))
//             .add_variable("b", Value::Int(10))
//             .add_variable("calc", crate::Value::Function(func.into()));
//         assert_eq!(eval_program!(r#"b.calc(a)"#, &mut ctx), Value::Int(20));
//     }
}
