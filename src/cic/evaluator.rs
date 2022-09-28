use crate::cic::checker::typecheck_closed;

use super::{ConstantName, GlobalEnvironment, Term};

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

    pub fn declare_assumption(&mut self, name: ConstantName, ty: Term) -> Term {
        if self.global.contains_declaration(&name) {
            panic!("A declaration with name {:?} already exists", name);
        }
        typecheck_closed(&ty, &self.global);
        self.global.declare_assumption(name.clone(), ty);
        Term::Constant(name)
    }

    pub fn declare_definition(&mut self, name: ConstantName, term: Term, ty: Term) {
        if self.global.contains_declaration(&name) {
            panic!("A declaration with name {:?} already exists", name);
        }
        let rty = typecheck_closed(&term, &self.global);
        assert_eq!(ty, rty);
        self.global.declare_definition(name, term, ty);
    }

    pub fn check(&self, term: &Term) {
        let ty = typecheck_closed(term, &self.global);
        println!("{}: {}", term.print(false), ty.print(false));
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
