use super::{ConstantName, Declaration, GlobalEnvironment, Term};

#[derive(Debug)]
pub struct Evaluator {
    global: GlobalEnvironment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            global: GlobalEnvironment::new(),
        }
    }

    pub fn declare_assumption(&mut self, name: ConstantName, ty: Term) {
        if self.global.contains_declaration(&name) {
            panic!("A declaration with name {:?} already exists", name);
        }
        self.global.declare_assumption(name, ty);
    }

    pub fn declare_definition(&mut self, name: ConstantName, term: Term, ty: Term) {
        if self.global.contains_declaration(&name) {
            panic!("A declaration with name {:?} already exists", name);
        }
        self.global.declare_definition(name, term, ty);
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
