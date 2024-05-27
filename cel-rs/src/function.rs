use crate::Val;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Function {
    pub name: &'static str,
    pub overloads: &'static [Overload],
}

type Func = fn(args: Vec<Val>) -> Val;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Overload {
    pub key: &'static str,
    pub func: Func
}