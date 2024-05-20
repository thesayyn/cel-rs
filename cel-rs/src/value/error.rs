use super::{value::{Value, Val}, ty::Ty};

#[derive(Eq, PartialEq)]
pub struct Error {
    id: Option<i64>,
    error: String,
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
}
