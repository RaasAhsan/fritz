use super::{GlobalEnvironment, LocalContext, Term};

pub fn typecheck(term: Term, global: &GlobalEnvironment, local: LocalContext) -> Term {
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
            if let Term::Forall(name, aty, rty) = typecheck(*t1, global, local.clone()) {
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

#[cfg(test)]
mod tests {
    use crate::cic::{
        syntax::{app, constant, constant_term, forall, function, type_term, var, var_term},
        GlobalEnvironment, LocalContext,
    };

    use super::typecheck;

    #[test]
    fn global_type() {
        let mut global = GlobalEnvironment::new();
        global.declare_assumption(constant("nat"), type_term());
        assert_eq!(
            typecheck(constant_term("nat"), &global, LocalContext::new()),
            type_term()
        );
    }

    #[test]
    fn global_object() {
        let mut global = GlobalEnvironment::new();
        global.declare_assumption(constant("nat"), type_term());
        global.declare_assumption(constant("z"), constant_term("nat"));
        assert_eq!(
            typecheck(constant_term("z"), &global, LocalContext::new()),
            constant_term("nat")
        );
    }

    #[test]
    fn local_type() {
        let mut global = GlobalEnvironment::new();
        global.declare_assumption(constant("nat"), type_term());
        global.declare_assumption(
            constant("id"),
            forall(
                var("A"),
                type_term(),
                function(var_term("A"), var_term("A")),
            ),
        );
        assert_eq!(
            typecheck(
                app(constant_term("id"), constant_term("nat")),
                &global,
                LocalContext::new()
            ),
            function(constant_term("nat"), constant_term("nat"))
        );
    }
}
