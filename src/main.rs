use wright::cic::{
    syntax::{constant, constant_term, function},
    Context, GlobalEnvironment, LocalContext, Term,
};

fn main() {
    let mut env = GlobalEnvironment::new();
    env.declare_assumption(constant("nat"), Term::Type);
    env.declare_assumption(constant("z"), constant_term("nat"));
    env.declare_assumption(
        constant("s"),
        function(constant_term("nat"), constant_term("nat")),
    );
}
