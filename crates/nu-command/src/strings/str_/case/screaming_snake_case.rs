use inflector::cases::screamingsnakecase::to_screaming_snake_case;

use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Example, PipelineData, ShellError, Signature, Span, Value};

use crate::action;
#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str screaming-snake-case"
    }

    fn signature(&self) -> Signature {
        Signature::build("str screaming-snake-case") /*.rest(
                                                         "rest",
                                                         SyntaxShape::ColumnPath,
                                                         "optionally convert text to SCREAMING_SNAKE_CASE by column paths",
                                                     )*/
    }

    fn usage(&self) -> &str {
        "converts a string to SCREAMING_SNAKE_CASE"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        input.map(
            move |val| action(&val, &to_screaming_snake_case),
            engine_state.ctrlc.clone(),
        )
    }
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert a string to camelCase",
                example: r#" "NuShell" | str screaming-snake-case"#,
                result: Some(Value::String {
                    val: "NU_SHELL".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: r#" "this_is_the_second_case" | str screaming-snake-case"#,
                result: Some(Value::String {
                    val: "THIS_IS_THE_SECOND_CASE".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: r#""this-is-the-first-case" | str screaming-snake-case"#,
                result: Some(Value::String {
                    val: "THIS_IS_THE_FIRST_CASE".to_string(),
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
