use super::{checker::typecheck, ConstantName, GlobalEnvironment, LocalContext, Term};

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
        typecheck(ty.clone(), &self.global, LocalContext::new());
        self.global.declare_assumption(name, ty);
    }

    pub fn declare_definition(&mut self, name: ConstantName, term: Term, ty: Term) {
        if self.global.contains_declaration(&name) {
            panic!("A declaration with name {:?} already exists", name);
        }
        let rty = typecheck(term.clone(), &self.global, LocalContext::new());
        assert_eq!(ty, rty);
        self.global.declare_definition(name, term, ty);
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
