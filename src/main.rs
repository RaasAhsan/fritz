use fritz::cic::{
    evaluator::Evaluator,
    syntax::{app, constant, forall, function, prop, var, var_term},
    Term,
};

fn main() {
    let mut eval = Evaluator::new();
    let type_nat = eval.declare_assumption(constant("nat"), Term::Type);
    let z = eval.declare_assumption(constant("z"), type_nat.clone());
    let s = eval.declare_assumption(constant("s"), function(type_nat.clone(), type_nat.clone()));
    let eqnat = eval.declare_assumption(
        constant("eqnat"),
        function(type_nat.clone(), function(type_nat.clone(), prop())),
    );
    let eqnat_refl = eval.declare_assumption(
        constant("eqnat_refl"),
        forall(
            var("n"),
            type_nat.clone(),
            app(app(eqnat.clone(), var_term("n")), var_term("n")),
        ),
    );
    eval.declare_definition(
        constant("eqnat_0"),
        app(eqnat_refl.clone(), z.clone()),
        app(app(eqnat.clone(), z.clone()), z.clone()),
    );
}
