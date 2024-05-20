use std::string::String as StdString;

use super::ty::Ty;
use super::value::{Val, Value};

pub struct String(StdString);

impl String {
    pub fn new(s: StdString) -> Self {
        Self(s)
    }
}

impl Value for String {
    fn ty(&self) -> Ty {
        Ty::String
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn compare(&self, other: &Val) -> Option<Val> {
        other.native_value().downcast_ref::<StdString>().map(|oths| {
            Val::from((&self.0).cmp(oths))
        })
    }
}
