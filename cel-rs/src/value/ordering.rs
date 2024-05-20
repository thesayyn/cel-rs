use std::cmp::Ordering;
use super::value::{Val, Value};


impl From<Ordering> for Val {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => Val::new_int(-1),
            Ordering::Equal => Val::new_int(0),
            Ordering::Greater => Val::new_int(1),
        }
    }
}

impl Into<Ordering> for Val {
    fn into(self) -> Ordering {
        match self.native_value().downcast_ref::<i64>() {
            Some(-1) => Ordering::Less,
            Some(0) => Ordering::Equal,
            Some(1) => Ordering::Greater,
            _ => panic!("invalid value for ordering")
        }
    }
}