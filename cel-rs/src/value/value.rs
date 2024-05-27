use std::cmp;
use std::collections::HashMap;
use std::{fmt, rc::Rc};

use crate::value::{error::Error, ty::Ty};

use super::bool::Bool;
use super::bytes::Bytes;
use super::double::Double;
use super::int::Int;
use super::map::Map;
use super::null::Null;
use super::string::String as CELString;
use super::uint::Uint;

pub trait Value {
    fn ty(&self) -> Ty;

    fn to_bool(&self) -> Val {
        Error::unimplemented(self.ty(), "to_bool")
    }

    fn to_type(&self, ty: Ty) -> Val {
        Error::invalid_conversion(self.ty(), ty)
    }

    fn native_value(&self) -> &dyn std::any::Any;

    fn compare(&self, other: &Val) -> Option<Val> {
        unimplemented!("compare {:?} {:?}", self.ty(), other.ty())
    }

    fn equals(&self, other: &Val) -> Val {
        unimplemented!("equals {:?} {:?}", self.ty(), other.ty())
    }
}

pub struct Val(Rc<dyn Value>);

impl cmp::PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.compare(&other).map(|v| v.into())
    }
}

impl std::hash::Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(format!("TODO:{:?}", &self).as_bytes());
    }
}

impl Eq for Val {}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        return self.equals(other).as_bool().expect("equals did not return bool").to_owned();
    }
}

impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Val(ty = {:?}", self.ty())?;
        // TODO: maybe replace this with a call to self.to_type(Ty::String).as_string()?
        match self.ty() {
            Ty::Bool => write!(f, ", value = {}", self.as_bool().unwrap()),
            Ty::Int => write!(
                f,
                ", value = {}",
                self.native_value().downcast_ref::<i64>().unwrap()
            ),
            Ty::UInt => write!(
                f,
                ", value = {}",
                self.native_value().downcast_ref::<u64>().unwrap()
            ),
            Ty::Double => write!(
                f,
                ", value = {}",
                self.native_value().downcast_ref::<f64>().unwrap()
            ),
            Ty::String => write!(
                f,
                ", value = {}",
                self.native_value().downcast_ref::<String>().unwrap()
            ),
            Ty::Bytes => write!(
                f,
                ", value = {:?}",
                self.native_value().downcast_ref::<Rc<Vec<u8>>>().unwrap()
            ),
            Ty::List => write!(f, ", value = TODO"),
            Ty::Map => write!(f, ", value = {:?}",  self.native_value().downcast_ref::<Rc<HashMap<Val, Val>>>().unwrap()),
            Ty::Null => write!(f, ", value = null"),
            Ty::Type => write!(
                f,
                ", value = {:?}",
                self.native_value().downcast_ref::<Ty>().unwrap()
            ),
            Ty::Unknown => write!(f, ", value = ?"),
            Ty::Error => write!(
                f,
                ", value = {}",
                self.native_value().downcast_ref::<Error>().unwrap()
            ),
            Ty::Dyn => write!(f, ", value = dyn"),
        }?;
        write!(f, ")")
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Val {
    pub fn new(v: impl Value + 'static) -> Self {
        Self(Rc::new(v))
    }

    pub fn new_bool(b: bool) -> Self {
        Self::new(Bool::new(b))
    }
    pub fn new_error(e: String) -> Self {
        Self::new(Error::new(e))
    }
    pub fn new_string(s: impl ToString) -> Self {
        Self::new(CELString::new(s.to_string()))
    }
    pub fn new_null() -> Self {
        Self::new(Null::new())
    }
    pub fn new_bytes(b: Rc<Vec<u8>>) -> Self {
        Self::new(Bytes::new(b))
    }
    pub fn new_double(f: f64) -> Self {
        Self::new(Double::new(f))
    }
    pub fn new_uint(u: u64) -> Self {
        Self::new(Uint::new(u))
    }
    pub fn new_int(i: i64) -> Self {
        Self::new(Int::new(i))
    }
    pub fn new_map(h: Rc<HashMap<Val, Val>>) -> Self {
        Self::new(Map::new(h))
    }
    pub fn new_list(b: Rc<Vec<Val>>) -> Self {
        Self::new(Null::new())
    }
    pub fn as_bool(&self) -> Option<&bool> {
        return self.0.native_value().downcast_ref::<bool>();
    }
    pub fn as_int(&self) -> Option<&i64> {
        return self.0.native_value().downcast_ref::<i64>();
    }
}

impl Value for Val {
    #[inline]
    fn ty(&self) -> Ty {
        self.0.ty()
    }

    #[inline]
    fn native_value(&self) -> &dyn std::any::Any {
        self.0.native_value()
    }

    #[inline]
    fn compare(&self, other: &Val) -> Option<Val> {
        self.0.compare(other)
    }

    #[inline]
    fn equals(&self, other: &Val) -> Val {
        self.0.equals(other)
    }

    #[inline]
    fn to_bool(&self) -> Val {
        self.0.to_bool()
    }

    #[inline]
    fn to_type(&self, ty: Ty) -> Val {
        self.0.to_type(ty)
    }
}
