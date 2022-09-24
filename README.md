# fritz

Fritz is an interactive theorem prover / proof assistant based on the Calculus of Inductive Constructions (CoIC), the formal system underlying the Coq proof assistant.

## Example: natural number equality

```rust
fn main() {
    let constant_nat = constant("nat");
    let type_nat = constant_term("nat");

    let mut eval = Evaluator::new();
    eval.declare_assumption(constant("nat"), Term::Type);
    eval.declare_assumption(constant("z"), type_nat.clone());
    eval.declare_assumption(
        constant("s"),
        function(constant_term("nat"), type_nat.clone()),
    );
    eval.declare_assumption(
        constant("eqnat"),
        function(type_nat.clone(), function(type_nat.clone(), prop())),
    );
    eval.declare_assumption(
        constant("eqnat_refl"),
        forall(
            var("n"),
            type_nat.clone(),
            app(app(constant_term("eqnat"), var_term("n")), var_term("n")),
        ),
    );
    eval.declare_definition(
        constant("eqnat_0"),
        app(constant_term("eqnat_refl"), constant_term("z")),
        app(
            app(constant_term("eqnat"), constant_term("z")),
            constant_term("z"),
        ),
    );
}
```
