use wright::cic::{
    evaluator::Evaluator,
    syntax::{constant, constant_term, function},
    Term,
};

fn main() {
    let mut eval = Evaluator::new();
    eval.declare_assumption(constant("nat"), Term::Type);
    eval.declare_assumption(constant("z"), constant_term("nat"));
    eval.declare_assumption(
        constant("s"),
        function(constant_term("nat"), constant_term("nat")),
    );
}
