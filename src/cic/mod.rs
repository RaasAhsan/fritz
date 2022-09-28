use std::{
    collections::{vec_deque::Iter, VecDeque},
    hash::Hash,
};

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

    pub fn print(&self, left: bool) -> String {
        match self {
            Term::Prop => "Prop".to_string(),
            Term::Type => "Type".to_string(),
            Term::Var(name) => name.0.clone(),
            Term::Constant(name) => name.0.clone(),
            Term::Forall(name, ty, body) => {
                let acc = if let Some(n) = name {
                    format!(
                        "forall {}: {}. {}",
                        n.0.clone(),
                        ty.print(false),
                        body.print(false)
                    )
                } else {
                    format!("{} -> {}", ty.print(true), body.print(false))
                };
                if left {
                    format!("({})", acc)
                } else {
                    acc
                }
            }
            Term::Abs(name, ty, body) => {
                if let Some(n) = name {
                    format!(
                        "\\{}: {} -> {}",
                        n.0.clone(),
                        ty.print(true),
                        body.print(false)
                    )
                } else {
                    format!("{} -> {}", ty.print(true), body.print(false))
                }
            }
            Term::App(t1, t2) => format!("({} {})", t1.print(false), t2.print(false)),
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
    declarations: VecDeque<(K, Declaration)>,
}

impl<K> Context<K> {
    pub fn new() -> Self {
        Context {
            declarations: VecDeque::new(),
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
        for (key, decl) in self.declarations.iter() {
            if key.eq(name) {
                return Some(decl);
            }
        }
        None
    }

    pub fn contains_declaration(&self, name: &K) -> bool {
        for (key, _) in self.declarations.iter() {
            if key.eq(name) {
                return true;
            }
        }
        false
    }

    fn add_declaration(&mut self, name: K, decl: Declaration) {
        self.declarations.push_front((name, decl));
    }

    pub fn iter<'a>(&'a self) -> ContextIter<'a, K> {
        ContextIter {
            inner: self.declarations.iter(),
        }
    }
}

pub struct ContextIter<'a, K> {
    inner: Iter<'a, (K, Declaration)>,
}

impl<'a, K> Iterator for ContextIter<'a, K> {
    type Item = &'a (K, Declaration);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K> DoubleEndedIterator for ContextIter<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

#[derive(Clone, Debug)]
pub enum Declaration {
    Assumption(Term),
    Definition(Term, Term),
}

trait Binding {
    fn name(&self) -> String;
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ConstantName(String);

impl Binding for ConstantName {
    fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VarName(String);

impl Binding for VarName {
    fn name(&self) -> String {
        self.0.clone()
    }
}
