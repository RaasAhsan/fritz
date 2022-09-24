use std::collections::HashMap;
use std::hash::Hash;

pub mod checker;
pub mod syntax;

#[derive(Clone, Debug)]
pub enum Term {
    Prop,
    Type,
    Var(VarName),
    Constant(ConstantName),
    Forall(Option<VarName>, Box<Term>, Box<Term>),
    Abs(Option<VarName>, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl From<ConstantName> for Term {
    fn from(name: ConstantName) -> Term {
        Term::Constant(name)
    }
}

#[derive(Debug)]
pub enum Sort {
    Prop,
    Type,
}

pub type GlobalEnvironment = Context<ConstantName>;
pub type LocalContext = Context<VarName>;

#[derive(Debug, Default)]
pub struct Context<K> {
    declarations: HashMap<K, Declaration>,
}

impl<K> Context<K> {
    pub fn new() -> Self {
        Context {
            declarations: HashMap::new(),
        }
    }
}

impl<K> Context<K>
where
    K: Eq + Hash,
{
    pub fn declare_assumption(&mut self, name: K, ty: Term) {
        self.add_declaration(name, Declaration::Assumption(ty));
    }

    pub fn declare_definition(&mut self, name: K, term: Term, ty: Term) {
        self.add_declaration(name, Declaration::Definition(term, ty));
    }

    pub fn get_declaration(&self, name: &K) -> Option<&Declaration> {
        self.declarations.get(name)
    }

    pub fn contains_declaration(&self, name: &K) -> bool {
        self.declarations.contains_key(name)
    }

    fn add_declaration(&mut self, name: K, decl: Declaration) {
        self.declarations.insert(name, decl);
    }
}

#[derive(Debug)]
pub enum Declaration {
    Assumption(Term),
    Definition(Term, Term),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ConstantName(String);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VarName(String);
