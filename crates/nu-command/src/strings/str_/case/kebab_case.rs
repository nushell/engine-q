use inflector::cases::kebabcase::to_kebab_case;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Example, PipelineData, ShellError, Signature, Span, Value};

use crate::action;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str kebab-case"
    }

    fn signature(&self) -> Signature {
        Signature::build("str kebab-case") /*.rest(
                                               "rest",
                                               SyntaxShape::ColumnPath,
                                               "optionally convert text to kebab-case by column paths",
                                           )*/
    }

    fn usage(&self) -> &str {
        "converts a string to kebab-case"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        input.map(
            move |val| action(&val, &to_kebab_case),
            engine_state.ctrlc.clone(),
        )
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert a string to kebab-case",
                example: "'NuShell' | str kebab-case",
                result: Some(Value::String {
                    val: "nu-shell".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to kebab-case",
                example: "'thisIsTheFirstCase' | str kebab-case",
                result: Some(Value::String {
                    val: "this-is-the-first-case".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to kebab-case",
                example: "'THIS_IS_THE_SECOND_CASE' | str kebab-case",
                result: Some(Value::String {
                    val: "this-is-the-second-case".to_string(),
                    span: Span::unknown(),
                }),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
