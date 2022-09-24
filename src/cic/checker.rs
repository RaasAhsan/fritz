use super::{Context, GlobalEnvironment, Term};

fn typecheck(term: Term, env: GlobalEnvironment) -> Term {
    match term {
        Term::Prop => Term::Type,
        Term::Type => Term::Type,
        Term::Var(name) => todo!(),
        Term::Constant(name) => match env.get_declaration(&name) {
            Some(declaration) => match declaration {
                super::Declaration::Assumption(ty) => ty.clone(),
                super::Declaration::Definition(_, ty) => ty.clone(),
            },
            None => panic!("Constant {:?} not found", name),
        },
        Term::Forall(_, _, _) => todo!(),
        Term::Abs(name, ty, term) => Term::Forall(name, term, ty),
        Term::App(_, _) => todo!(),
    }
}
