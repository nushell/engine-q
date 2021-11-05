use inflector::cases::camelcase::to_camel_case;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value};

use crate::operate;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str camel-case"
    }

    fn signature(&self) -> Signature {
        Signature::build("str camel-case").rest(
            "rest",
            SyntaxShape::CellPath,
            "optionally convert text to camelCase by column paths",
        )
    }

    fn usage(&self) -> &str {
        "converts a string to camelCase"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        operate(engine_state, stack, call, input, &to_camel_case)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert a string to camelCase",
                example: " 'NuShell' | str camel-case",
                result: Some(Value::String {
                    val: "nuShell".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: "'this-is-the-first-case' | str camel-case",
                result: Some(Value::String {
                    val: "thisIsTheFirstCase".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: " 'this_is_the_second_case' | str camel-case",
                result: Some(Value::String {
                    val: "thisIsTheSecondCase".to_string(),
                    span: Span::unknown(),
                }),
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}