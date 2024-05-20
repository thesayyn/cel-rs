use super::value::Value;

// https://github.com/google/cel-spec/blob/master/doc/langdef.md#values
#[derive(Eq, PartialEq, Debug)]
pub enum Ty {
    Int,   
    UInt,
    Double,
    Bool,
    String,
    Bytes,
    List,
    Map,
    Null,
    // these should be here?
    Type,
    Unknown,
    Error,
    Dyn,
}


impl ToString for Ty {
    fn to_string(&self) -> String {
        String::from(match self {
            Ty::Int => "int",
            Ty::UInt => "uint",
            Ty::Double => "double",
            Ty::Bool => "bool",
            Ty::String => "string",
            Ty::Bytes => "bytes",
            Ty::List => "list",
            Ty::Map => "map",
            Ty::Null => "null_type",
            Ty::Type => "type",
            Ty::Unknown => "unknown",
            Ty::Error => "error",
            Ty::Dyn => "dyn",
        })
    }
}

impl Value for Ty {
    fn ty(&self) -> Self {
       Self::Type
    }
    
    fn native_value(&self) -> &dyn std::any::Any {
       self
    }
}