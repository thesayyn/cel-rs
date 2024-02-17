use crate::{value::Value, Context};
use parser::{ArithmeticOp, Atom, Expression, RelationOp};

impl From<Atom> for Value {
    fn from(atom: Atom) -> Self {
        match atom {
            Atom::Int(i) => Value::Int(i),
            Atom::UInt(ui) => Value::UInt(ui),
            Atom::Float(f) => Value::Float(f),
            Atom::Bool(b) => Value::Bool(b),
            Atom::Null => Value::Null,
            Atom::Bytes(b) => Value::Bytes(b),
            Atom::String(s) => Value::String(s),
        }
    }
}

pub struct Eval {
    ctx: Context,
}

impl Eval {
    pub fn new(ctx: Context) -> Self {
        Eval { ctx }
    }
    pub fn eval(&self, expr: Expression) -> Value {
        match expr {
            Expression::Atom(atom) => atom.into(),
            Expression::Relation(left, op, right) => {
                let left = self.eval(*left);
                let right = self.eval(*right);
                let result = match op {
                    RelationOp::Equals => left.eq(&right),
                    RelationOp::LessThan => todo!("lt"),
                    RelationOp::LessThanEq => todo!("lte"),
                    RelationOp::GreaterThan => todo!("gt"),
                    RelationOp::GreaterThanEq => todo!("gte"),
                    RelationOp::NotEquals => todo!("ne"),
                    RelationOp::In => todo!("in"),
                };
                Value::Bool(result)
            }
            Expression::Arithmetic(left, op, right) => {
                let left = self.eval(*left);
                let right = self.eval(*right);
                match op {
                    ArithmeticOp::Add => left + right,
                    ArithmeticOp::Subtract => left - right,
                    ArithmeticOp::Divide => left / right,
                    ArithmeticOp::Multiply => left * right,
                    ArithmeticOp::Modulus => todo!("modulus"),
                }
            }
            Expression::Ternary(_, _, _) => todo!(),
            Expression::Or(_, _) => todo!(),
            Expression::And(_, _) => todo!(),
            Expression::Unary(_, _) => todo!(),
            Expression::Member(_, _) => todo!(),
            Expression::List(_) => todo!(),
            Expression::Map(_) => todo!(),
            Expression::Ident(_) => todo!(),
        }
    }
}
