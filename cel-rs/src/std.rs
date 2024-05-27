use crate::{function::{Function, Overload}, Val};


fn invoke_dyn(args: Vec<Val>) -> Val {
    args.first().unwrap().clone()
}

pub fn new_dyn() -> Function {
    Function {
        name: "dyn",
        overloads: &[Overload {
            key: "dyn",
            func: invoke_dyn,
        }],
    }
}


