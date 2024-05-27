use crate::{function::Function, value::value::Val};
use std::{collections::HashMap, rc::Rc};

pub struct Context {
    par: Option<Rc<Context>>,
    variables: HashMap<&'static str, Val>,
    funtions: HashMap<&'static str, Function>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            par: Default::default(),
            variables: Default::default(),
            funtions: HashMap::from([
                ("dyn", crate::std::new_dyn())
            ]),
        }
    }
}

impl Context {
    pub fn add_variable(&mut self, name: &'static str, val: Val) -> &mut Self {
        self.variables.insert(name, val);
        self
    }
    pub fn resolve_variable(&self, name: &String) -> Option<&Val> {
        self.variables.get(name.as_str())
    }

    pub fn add_function(&mut self, name: &'static str, func: Function) -> &mut Self {
        self.funtions.insert(name, func);
        self
    }
    pub fn resolve_function(&self, name: &String) -> Option<&Function> {
        self.funtions.get(name.as_str())
    }

    pub fn parent(&self) -> Option<Rc<Context>> {
        self.par.clone()
    }
}
