use super::{
    ty::Ty,
    value::{Val, Value},
};

#[derive(Eq, PartialEq)]
pub struct Bool(bool);

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Bool {
    pub fn new(b: bool) -> Self {
        Self(b)
    }
}

impl Value for Bool {
    fn ty(&self) -> Ty {
        Ty::Bool
    }

    fn to_bool(&self) -> Val {
        Val::new(Bool::from(self.0))
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn compare(&self, other: &Val) -> Option<Val> {
        other.as_bool().map(|ob| (&self.0).cmp(ob).into())
    }

    fn equals(&self, other: &Val) -> Val {
        other
            .as_bool()
            .map(|f| Val::new_bool(&self.0 == f))
            .unwrap_or(Val::new_bool(false))
    }
}
