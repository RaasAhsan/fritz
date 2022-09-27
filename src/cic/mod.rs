use std::collections::HashMap;
use std::hash::Hash;

pub mod checker;
pub mod evaluator;
pub mod syntax;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Prop,
    Type,
    Var(VarName),
    Constant(ConstantName),
    Forall(Option<VarName>, Box<Term>, Box<Term>),
    Abs(Option<VarName>, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    fn substitute_var(&mut self, name: VarName, term: &Term) {
        match self {
            Term::Var(n) if *n == name => *self = term.clone(),
            Term::Forall(_, t1, t2) => {
                t1.substitute_var(name.clone(), term);
                t2.substitute_var(name, term);
            }
            Term::Abs(_, t1, t2) => {
                t1.substitute_var(name.clone(), term);
                t2.substitute_var(name, term);
            }
            Term::App(t1, t2) => {
                t1.substitute_var(name.clone(), term);
                t2.substitute_var(name, term);
            }
            _ => {}
        }
    }
}

impl From<ConstantName> for Term {
    fn from(name: ConstantName) -> Term {
        Term::Constant(name)
    }
}

impl From<VarName> for Term {
    fn from(name: VarName) -> Term {
        Term::Var(name)
    }
}

#[derive(Debug)]
pub enum Sort {
    Prop,
    Type,
}

pub type GlobalEnvironment = Context<ConstantName>;
pub type LocalContext = Context<VarName>;

/// A context represents a global environment or a local context.
#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
pub enum Declaration {
    Assumption(Term),
    Definition(Term, Term),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ConstantName(String);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VarName(String);
