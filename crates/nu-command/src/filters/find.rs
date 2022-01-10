use nu_engine::{eval_block, CallExt};
use nu_protocol::{
    ast::Call,
    engine::{CaptureBlock, Command, EngineState, Stack},
    Category, Example, IntoInterruptiblePipelineData, IntoPipelineData, PipelineData, ShellError,
    Signature, Span, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Find;

impl Command for Find {
    fn name(&self) -> &str {
        "find"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "predicate",
                SyntaxShape::Block(Some(vec![SyntaxShape::Any])),
                "the predicate to satisfy",
                Some('p'),
            )
            .rest("rest", SyntaxShape::Any, "terms to search")
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Searches terms in the input or for an element of the input that satisfies the predicate."
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Search for multiple terms in a command output",
                example: r#"ls | find toml md sh"#,
                result: None,
            },
            Example {
                description: "Search for a term in a string",
                example: r#"echo Cargo.toml | find toml"#,
                result: Some(Value::List {
                    vals: vec![Value::test_string("Cargo.toml".to_owned())],
                    span: Span::test_data()
                })
            },
            Example {
                description: "Search a number or a file size in a list of numbers",
                example: r#"[1 5 3kb 4 3Mb] | find 5 3kb"#,
                result: Some(Value::List {
                    vals: vec![Value::test_int(5), Value::test_filesize(3000)],
                    span: Span::test_data()
                }),
            },
            Example {
                description: "Search a char in a list of string",
                example: r#"[moe larry curly] | find l"#,
                result: Some(Value::List {
                    vals: vec![Value::test_string("larry"), Value::test_string("curly")],
                    span: Span::test_data()
                })
            },
            Example {
                description: "Find the first odd value",
                example: "echo [2 4 3 6 8] | find --predicate { ($it mod 2) == 1 }",
                result: None,
            },
            Example {
                description: "Find if a service is not running",
                example: "echo [[version patch]; [0.1.0 $false] [0.1.1 $true] [0.2.0 $false]] | find -p { $it.patch }",
                result: Some(Value::Record {
                    cols: vec!["version".to_owned(), "patch".to_owned()],
                    vals: vec![Value::test_string("0.1.1"), Value::test_bool(true)],
                    span: Span::test_data(),
                }),
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

        let ctrlc = engine_state.ctrlc.clone();
        let engine_state = engine_state.clone();

        match call.get_flag::<CaptureBlock>(&engine_state, stack, "predicate")? {
            Some(predicate) => {
                let capture_block = predicate;
                let block_id = capture_block.block_id;

                let block = engine_state.get_block(block_id).clone();
                let var_id = block.signature.get_positional(0).and_then(|arg| arg.var_id);

                let mut stack = stack.captures_to_stack(&capture_block.captures);

                Ok(input
                    .into_interruptible_iter(ctrlc)
                    .find(move |value| {
                        if let Some(var_id) = var_id {
                            stack.add_var(var_id, value.clone());
                        }

                        eval_block(&engine_state, &mut stack, &block, PipelineData::new(span))
                            .map_or(false, |pipeline_data| {
                                pipeline_data.into_value(span).is_true()
                            })
                    })
                    .unwrap_or(Value::Nothing { span })
                    .into_pipeline_data())
            }
            None => {
                let terms = call.rest::<Value>(&engine_state, stack, 0)?;
                Ok(input
                    .into_iter()
                    .filter(move |value| {
                        terms.iter().any(|term| match value {
                            Value::Bool { .. }
                            | Value::Int { .. }
                            | Value::Filesize { .. }
                            | Value::Duration { .. }
                            | Value::Date { .. }
                            | Value::Range { .. }
                            | Value::Float { .. }
                            | Value::Block { .. }
                            | Value::Nothing { .. }
                            | Value::Error { .. } => {
                                value.eq(span, term).map_or(false, |value| value.is_true())
                            }
                            Value::String { .. }
                            | Value::List { .. }
                            | Value::CustomValue { .. } => term
                                .r#in(span, value)
                                .map_or(false, |value| value.is_true()),
                            Value::Record { vals, .. } => vals.iter().any(|val| {
                                term.r#in(span, val).map_or(false, |value| value.is_true())
                            }),
                            Value::Binary { .. } => todo!(),
                            Value::CellPath { .. } => todo!(),
                        })
                    })
                    .into_pipeline_data(ctrlc))
            }
        }
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
