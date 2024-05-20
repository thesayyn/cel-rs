use std::cmp::Ordering;
use super::value::{Val, Value};
use super::ty::Ty;

lazy_static::lazy_static! {
    pub static ref NULL: Null = Null::new();
}

pub struct Null{}

impl Null {
    pub fn new() -> Self {
        Self {}
    }
}

impl Value for Null {
    fn ty(&self) -> Ty {
        Ty::Null
    }

    fn native_value(&self) -> &dyn std::any::Any {
      &()
    }
    fn compare(&self, other: &Val) -> Option<Val> {
        if other.ty() == Ty::Null {
            return Some(Val::from(Ordering::Equal))
        }
        None
    }
}
