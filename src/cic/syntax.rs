use super::{ConstantName, Term, VarName};

pub fn constant<T: AsRef<str>>(name: T) -> ConstantName {
    ConstantName(name.as_ref().to_string())
}

pub fn constant_term<T: AsRef<str>>(name: T) -> Term {
    constant(name).into()
}

pub fn prop() -> Term {
    Term::Prop
}

pub fn type_term() -> Term {
    Term::Type
}

pub fn forall(name: VarName, ty: Term, term: Term) -> Term {
    Term::Forall(Some(name), Box::new(ty), Box::new(term))
}

pub fn function(ty: Term, term: Term) -> Term {
    Term::Forall(None, Box::new(ty), Box::new(term))
}
