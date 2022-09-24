use super::{GlobalEnvironment, LocalContext, Term, VarName};

fn typecheck(term: Term, global: GlobalEnvironment, local: LocalContext) -> Term {
    match term {
        Term::Prop => Term::Type,
        Term::Type => Term::Type,
        Term::Var(name) => match local.get_declaration(&name) {
            Some(declaration) => match declaration {
                super::Declaration::Assumption(ty) => ty.clone(),
                super::Declaration::Definition(_, ty) => ty.clone(),
            },
            None => panic!("Variable {:?} not found", name),
        },
        Term::Constant(name) => match global.get_declaration(&name) {
            Some(declaration) => match declaration {
                super::Declaration::Assumption(ty) => ty.clone(),
                super::Declaration::Definition(_, ty) => ty.clone(),
            },
            None => panic!("Constant {:?} not found", name),
        },
        Term::Forall(_, _, _) => Term::Prop, // TODO
        Term::Abs(name, ty, term) => {
            let mut next_local = local; // TODO: ideally we don't clone, just chain
            if let Some(n) = &name {
                next_local.declare_assumption(n.clone(), *ty.clone());
            }
            let tty = typecheck(*term, global, next_local);
            Term::Forall(name, ty, Box::new(tty))
        }
        Term::App(t1, t2) => {
            if let Term::Forall(name, aty, rty) = typecheck(*t1, global.clone(), local.clone()) {
                let t2ty = typecheck(*t2.clone(), global, local);
                if t2ty == *aty {
                    let mut tty = rty;
                    if let Some(n) = name {
                        tty.substitute_var(n, *t2);
                    }
                    *tty
                } else {
                    panic!("Types did not match");
                }
            } else {
                panic!("Cannot apply to a non-abstraction");
            }
        }
    }
}
