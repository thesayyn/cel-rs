
use super::{ty::Ty, value::{Val, Value}};

pub struct Double(f64);

impl Double {
    pub fn new(f: f64) -> Self {
        Self(f)
    }
}

impl Value for Double {
    fn ty(&self) -> super::ty::Ty {
        Ty::Double
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn equals(&self, other: &Val) -> Val {
        Val::new_bool(
            other
                .native_value()
                .downcast_ref::<f64>()
                .is_some_and(|f| f.eq(&self.0)),
        )
    }
    
    fn compare(&self, other: &Val) -> Option<Val> {
        let vl = other.native_value().downcast_ref::<f64>();
        if vl.is_some() {
            return (&self.0).partial_cmp(vl.unwrap()).map(|f| f.into())
        }
        None
    }
}
