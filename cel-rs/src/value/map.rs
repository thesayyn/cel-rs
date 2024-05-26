use super::ty::Ty;
use crate::{Val, Value};
use std::{collections::HashMap, rc::Rc};

pub struct Map(Rc<HashMap<Val, Val>>);

impl Map {
    pub fn new(h: Rc<HashMap<Val, Val>>) -> Self {
        Self(h)
    }
}

impl Value for Map {
    fn ty(&self) -> super::ty::Ty {
        Ty::Map
    }

    fn native_value(&self) -> &dyn std::any::Any {
        &self.0
    }

    fn equals(&self, other: &Val) -> Val {
        other
            .native_value()
            .downcast_ref::<Rc<HashMap<Val, Val>>>()
            .map(|other| {
                if other.len() != self.0.len() {
                    return Val::new_bool(false);
                }

                for (k, v) in self.0.iter() {
                    let ov = other.get(k);
                    if let Some(ov) = ov {
                        // TODO: use value.equals once all types support it.
                        if !ov.compare(v).is_some_and(|o| o.as_int() == Some(&0)) {
                            return Val::new_bool(false);
                        }
                    } else {
                        return Val::new_bool(false);
                    }
                }

                Val::new_bool(true)
            })
            .unwrap_or(Val::new_bool(false))
    }
}
