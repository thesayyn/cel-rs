use core::fmt;

use super::{
    ty::Ty,
    value::{Val, Value},
};

#[derive(Eq, PartialEq)]
pub struct Error {
    id: Option<i64>,
    error: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error(id = {:?}, message = {})", self.id, self.error)
    }
}

impl Error {
    pub fn new(error: String) -> Val {
        return Val::new(Self { id: None, error });
    }
    pub fn unimplemented(ty: Ty, f: &str) -> Val {
        Self::new(format!("{} does not implement {}", ty.to_string(), f))
    }
    pub fn invalid_conversion(from_ty: Ty, to_ty: Ty) -> Val {
        Self::new(format!(
            "type {} could not be converted to {}",
            from_ty.to_string(),
            to_ty.to_string()
        ))
    }
}

impl Value for Error {
    fn ty(&self) -> Ty {
        Ty::Error
    }

    fn native_value(&self) -> &dyn std::any::Any {
        self
    }

    fn equals(&self, other: &Val) -> Val {
        if other.ty() != Ty::Error {
            return Val::new_bool(false);
        }

        Val::new_bool(
            other
            .native_value()
            .downcast_ref::<Self>()
            .map(|oerr| oerr.eq(self))
            .is_some_and(|f| f)
        )
    }
}
