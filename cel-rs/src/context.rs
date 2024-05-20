use std::{collections::HashMap, rc::Rc};
use crate::value::value::{Val};

#[derive(Default)]
pub struct Context {
    par: Option<Rc<Context>>,
    vars: HashMap<&'static str, Val>
}

impl Context {
    pub fn add_variable(mut self, name: &'static str, val: Val) -> Self {
        self.vars.insert(name, val);
        self
    }
    pub fn resolve(&self, name: &String) -> Option<&Val> {
        self.vars.get(name.as_str())
    }
    pub fn parent(&self) -> Option<Rc<Context>> {
       self.par.clone()
    }
}