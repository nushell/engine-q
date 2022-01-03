use nu_engine::{eval_block, CallExt};
use nu_protocol::{
    ast::Call,
    engine::{CaptureBlock, Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, SyntaxShape,
    Value,
};

#[derive(Clone)]
pub struct Find;

impl Command for Find {
    fn name(&self) -> &str {
        "find"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required(
                "predicate",
                SyntaxShape::RowCondition,
                "the predicate to satisfy",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Searches for an element of the input that satisfies the predicate."
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Find if a service is not running",
                example: "echo [[version patch]; [0.1.0 $false] [0.1.1 $true] [0.2.0 $false]] | find $it.patch",
                result: Some(Value::Record {
                    cols: vec!["version".to_owned(), "patch".to_owned()],
                    vals: vec![Value::test_string("0.1.1"), Value::test_bool(true)],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Find the first odd value",
                example: "echo [2 4 3 6 8] | find ($it mod 2) == 1",
                result: Some(Value::test_int(3)),
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
        let span = call.head;

        let capture_block: CaptureBlock = call.req(engine_state, stack, 0)?;
        let block_id = capture_block.block_id;

        let block = engine_state.get_block(block_id).clone();
        let var_id = block.signature.get_positional(0).and_then(|arg| arg.var_id);

        let mut stack = stack.captures_to_stack(&capture_block.captures);

        let ctrlc = engine_state.ctrlc.clone();
        let engine_state = engine_state.clone();

        Ok(
            match input.into_interruptible_iter(ctrlc).find(move |value| {
                if let Some(var_id) = var_id {
                    stack.add_var(var_id, value.clone());
                }

                eval_block(&engine_state, &mut stack, &block, PipelineData::new(span))
                    .map_or(false, |pipeline_data| {
                        pipeline_data.into_value(span).is_true()
                    })
            }) {
                Some(found_value) => found_value,
                None => Value::Nothing { span },
            }
            .into_pipeline_data(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Find)
    }
}
