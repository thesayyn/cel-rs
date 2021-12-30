use parser::{ArithmeticOp, Atom, Expression, RelationOp};
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    Null,
    Bool(bool),
    Bytes(Rc<Vec<u8>>),
    String(Rc<String>),
}

impl From<&Atom> for Value {
    fn from(atom: &Atom) -> Self {
        match atom {
            Atom::Int(i) => Value::Int(*i),
            Atom::UInt(ui) => Value::UInt(*ui),
            Atom::Float(f) => Value::Float(*f),
            Atom::Bool(b) => Value::Bool(*b),
            Atom::Bytes(b) => Value::Bytes(b.clone()),
            Atom::Null => Value::Null,
            Atom::String(s) => Value::String(s.clone()),
        }
    }
}

impl<'a> Value {
    pub fn from(expr: &'a Expression) -> Value {
        match expr {
            Expression::Atom(atom) => atom.into(),
            Expression::Relation(left, op, right) => {
                let left = Value::from(left);
                let right = Value::from(right);
                let result = match op {
                    RelationOp::Equals => left.eq(&right),
                    _ => unimplemented!(),
                };
                Value::Bool(result)
            }
            Expression::Arithmetic(left, op, right) => {
                let left = Value::from(left);
                let right = Value::from(right);
                match op {
                    ArithmeticOp::Add => left + right,
                    ArithmeticOp::Subtract => left - right,
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
            _ => todo!("add"),
        }
    }
}

impl std::ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l - r),
            _ => todo!("sub"),
        }
    }
}
