use super::{
    ty::Ty,
    value::{Val, Value},
};

pub struct Uint(u64);

impl Uint {
    pub fn new(u: u64) -> Self {
        Self(u)
    }
}

impl Value for Uint {
    fn ty(&self) -> Ty {
        Ty::UInt
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn compare(&self, other: &Val) -> Option<Val> {
        other
            .native_value()
            .downcast_ref::<u64>()
            .map(|oui| Val::from((&self.0).cmp(oui)))
    }
}
