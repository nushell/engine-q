use inflector::cases::camelcase::to_camel_case;

use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Example, PipelineData, ShellError, Signature, Span, Value};

use crate::action;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str camel-case"
    }

    fn signature(&self) -> Signature {
        Signature::build("str camel-case") /*.rest(
                                               "rest",
                                               SyntaxShape::ColumnPath,
                                               "optionally convert text to camelCase by column paths",
                                           )*/
    }

    fn usage(&self) -> &str {
        "converts a string to camelCase"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        input.map(
            move |val| action(&val, &to_camel_case),
            engine_state.ctrlc.clone(),
        )
    }
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert a string to camelCase",
                example: r#" "NuShell" | str camel-case"#,
                result: Some(Value::String {
                    val: "nuShell".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: r#" "this_is_the_second_case" | str camel-case"#,
                result: Some(Value::String {
                    val: "thisIsTheSecondCase".to_string(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a string to camelCase",
                example: r#""this-is-the-first-case" | str camel-case"#,
                result: Some(Value::String {
                    val: "thisIsTheFirstCase".to_string(),
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

// #[cfg(test)]
// mod tests {
//     use super::{to_camel_case, SubCommand};

//     #[test]
//     fn examples_work_as_expected() -> Result<(), ShellError> {
//         use crate::examples::test as test_examples;

//         test_examples(SubCommand {})
//     }

//     #[test]
//     fn camel_case_from_kebab() {
//         let word = string("this-is-the-first-case");
//         let expected = string("thisIsTheFirstCase");

//         let actual = action(&word, Tag::unknown(), &to_camel_case).unwrap();
//         assert_eq!(actual, expected);
//     }
//     #[test]
//     fn camel_case_from_snake() {
//         let word = string("this_is_the_second_case");
//         let expected = string("thisIsTheSecondCase");

//         let actual = action(&word, Tag::unknown(), &to_camel_case).unwrap();
//         assert_eq!(actual, expected);
//     }
// }
