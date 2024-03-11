use std::{collections::HashMap, rc::Rc};

use crate::{eval::Bag, Value};

#[derive(Default)]
pub struct Context {
    par: Option<Rc<Context>>,
    vars: HashMap<&'static str, Value>
}

impl Bag for Context {
    fn unpack(self) -> Value {
        todo!()
    }
}

impl Context {
    pub fn add_variable(mut self, name: &'static str, val: Value) -> Self {
        self.vars.insert(name, val);
        self
    }

    pub fn resolve(&self, name: &String) -> Option<Value> {
        // TODO: this is probably expensive to do.
        self.vars.get(name.as_str()).map(|f| f.to_owned())
    }
    pub fn parent(&self) -> Option<Rc<Context>> {
       self.par.clone()
    }
}