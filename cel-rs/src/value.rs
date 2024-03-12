use std::{hash::Hash, rc::Rc};
use ordered_float::OrderedFloat;
use ordered_hash_map::OrderedHashMap;
use crate::parser::Atom;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Value {
    Int(i64),   
    UInt(u64),
    // We need to use a wrapper for floats for Eq, Hash
    Float(OrderedFloat<f64>),
    Null,
    Bool(bool),
    Bytes(Rc<Vec<u8>>),
    String(Rc<String>),
    Map(Rc<OrderedHashMap<Value, Value>>),
    List(Rc<Vec<Value>>),
    Function(Rc<FnValue>, Option<Rc<Value>>)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FnValue {
    pub name: &'static str,
    pub overloads: &'static [Overload],
}

pub type Func = fn(args: Vec<Value>) -> Value;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Overload {
    pub key: &'static str,
    pub func: Func
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Int(v) => v != 0,
            Value::UInt(v) => v != 0,
            Value::Float(v) => v != 0.0,
            Value::Null => false,
            Value::Bool(v) => v,
            Value::Bytes(v) => v.len() > 0,
            Value::String(v) => v.len() > 0,
            Value::Map(v) => v.len() > 0,
            Value::List(v) => v.len() > 0,
            Value::Function(_, _) => true,
        }
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO: this is not right. hash each arm separately.
        core::mem::discriminant(self).hash(state);
    }
}

impl From<Atom> for Value {
    fn from(atom: Atom) -> Self {
        match atom {
            Atom::Int(i) => Value::Int(i),
            Atom::UInt(ui) => Value::UInt(ui),
            Atom::Float(f) => Value::Float(f.into()),
            Atom::Bool(b) => Value::Bool(b),
            Atom::Null => Value::Null,
            Atom::Bytes(b) => Value::Bytes(b),
            Atom::String(s) => Value::String(s),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "int({})", v),
            Value::UInt(v) => write!(f, "uint({})", v),
            Value::Float(v) => write!(f, "float({})", v),
            Value::Null => write!(f, "null"),
            Value::Bool(v) => write!(f, "bool({})", v),
            Value::Bytes(v) => write!(f, "bytes(len = {})", v.len()),
            Value::String(v) => write!(f, "string({})", v),
            Value::Map(v) => write!(f, "map(len = {})", v.len()),
            Value::List(v) => write!(f, "list(len = {})", v.len()),
            Value::Function(v, bound) => write!(f, "function(name = {}, bound = {:?})", v.name, bound),
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
