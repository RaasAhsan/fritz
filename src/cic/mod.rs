pub mod syntax;

#[derive(Debug)]
pub enum Term {
    Prop,
    Type,
    Var(u16),
    Forall(Box<Term>, Box<Term>),
    Abs(Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Debug)]
pub enum Sort {
    Prop,
    Type,
}

#[derive(Debug)]
pub struct GlobalEnv {
    declarations: Vec<(Constant, GlobalDecl)>,
}

#[derive(Debug)]
pub enum GlobalDecl {
    Assumption(Term),
    Definition(Term, Term),
}

#[derive(Debug, PartialEq)]
pub struct Constant(String);
