use std::rc::Rc;
use super::{ty::Ty, value::{Val, Value}};

pub struct Bytes(Rc<Vec<u8>>);


impl Bytes {
    pub fn new(b: Rc<Vec<u8>>) -> Self {
        Self(b)
    }
}

impl Value for Bytes {
    fn ty(&self) -> super::ty::Ty {
        Ty::Bytes
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn compare(&self, other: &Val) -> Option<Val> {
        other.native_value().downcast_ref::<Rc<Vec<u8>>>().map(|ob| {
            (&self.0).cmp(ob).into()
        })
    }
}