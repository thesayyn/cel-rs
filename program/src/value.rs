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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "int({})", v),
            Value::UInt(v) => write!(f, "uint({})", v),
            Value::Float(v) => write!(f, "float({})", v),
            Value::Null => write!(f, "null"),
            Self::Bool(v) => write!(f, "bool({})", v),
            Value::Bytes(v) => write!(f, "bytes(len = {})", v.len()),
            Value::String(v) => write!(f, "string({})", v),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
            (a, b) => todo!("mul {} {}", a, b),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l / r),
            (a, b) => todo!("div {} {}", a, b),
        }
    }
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
            (a, b) => todo!("add {} {}", a, b),
        }
    }
}

impl std::ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l - r),
            (a, b) => todo!("sub {} {}", a, b),
        }
    }
}
