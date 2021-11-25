use nu_engine::eval_expression;
use nu_protocol::ast::{Call, Expr, Expression};
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::IntoPipelineData;
use nu_protocol::{Category, Example, PipelineData, ShellError, Signature, SyntaxShape};

#[derive(Clone)]
pub struct All;

impl Command for All {
    fn name(&self) -> &str {
        "all?"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required(
                "predicate",
                SyntaxShape::RowCondition,
                "the predicate that must match",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Test if every element of the input matches a predicate."
    }

    fn examples(&self) -> Vec<Example> {
        use nu_protocol::Value;

        vec![
            Example {
                description: "Find if services are running",
                example: "echo [[status]; [UP] [UP]] | all? status == UP",
                result: Some(Value::from(true)),
            },
            Example {
                description: "Check that all values are even",
                example: "echo [2 4 6 8] | all? ($it mod 2) == 0",
                result: Some(Value::from(true)),
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let predicate = &call.positional[0];

        let (var_id, expr) = match predicate {
            Expression {
                expr: Expr::RowCondition(var_id, expr),
                ..
            } => (*var_id, expr.clone()),
            _ => return Err(ShellError::InternalError("Expected row condition".into())),
        };

        let ctrlc = engine_state.ctrlc.clone();
        let engine_state = engine_state.clone();

        // FIXME: Expensive clone. I would need a way to collect the captures of the `RowCondition`.
        let mut stack = stack.clone();

        Ok(input
            .into_interruptible_iter(ctrlc)
            .all(move |value| {
                stack.add_var(var_id, value);
                eval_expression(&engine_state, &mut stack, &expr).map_or(false, |v| v.is_true())
            })
            .into_pipeline_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(All {})
    }
}
