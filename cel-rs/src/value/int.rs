use super::{ty::Ty, value::Val, value::Value};

pub struct Int(i64);

impl Int {
    pub fn new(i: i64) -> Self {
        Self(i)
    }
}

impl Value for Int {
    fn ty(&self) -> Ty {
        Ty::Int
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn compare(&self, other: &Val) -> Option<Val> {
        other
            .native_value()
            .downcast_ref::<i64>()
            .map(|oi| {
                (&self.0).cmp(oi).into()
            })
    }
}
