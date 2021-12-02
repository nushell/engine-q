use nu_engine::CallExt;
use nu_protocol::{
    ast::{Call, CellPath},
    engine::{Command, EngineState, Stack},
    Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "into decimal"
    }

    fn signature(&self) -> Signature {
        Signature::build("into decimal").rest(
            "rest",
            SyntaxShape::CellPath,
            "optionally convert text into decimal by column paths",
        )
    }

    fn usage(&self) -> &str {
        "converts text into decimal"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        operate(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Convert string to integer in table",
                example: "[[num]; ['5.01']] | into decimal num",
                result: Some(Value::List {
                    vals: vec![Value::Record {
                        cols: vec!["num".to_string()],
                        vals: vec![Value::test_float(5.01)],
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "Convert string to integer",
                example: "'1.345' | into decimal",
                result: Some(Value::test_float(1.345)),
            },
            Example {
                description: "Convert decimal to integer",
                example: "'-5.9' | into decimal",
                result: Some(Value::test_float(-5.9)),
            },
        ]
    }
}

fn operate(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let head = call.head;
    let column_paths: Vec<CellPath> = call.rest(engine_state, stack, 0)?;

    input.map(
        move |v| {
            if column_paths.is_empty() {
                action(&v, head)
            } else {
                let mut ret = v;
                for path in &column_paths {
                    let r =
                        ret.update_cell_path(&path.members, Box::new(move |old| action(old, head)));
                    if let Err(error) = r {
                        return Value::Error { error };
                    }
                }

                ret
            }
        },
        engine_state.ctrlc.clone(),
    )
}

fn action(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val: s, .. } => {
            let other = s.trim();

            match other.parse::<f64>() {
                Ok(x) => Value::Float { val: x, span: head },
                Err(reason) => Value::Error {
                    error: ShellError::CantConvert("".to_string(), reason.to_string(), head),
                },
            }
        }
        other => {
            let got = format!("Expected a string, got {}", other.get_type());
            Value::Error {
                error: ShellError::UnsupportedInput(got, head),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nu_protocol::Type::Error;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn turns_to_integer() {
        let word = Value::test_string("3.1415");
        let expected = Value::test_float(3.1415);

        let actual = action(&word, Span::unknown());
        assert_eq!(actual, expected);
    }

    #[test]
    fn communicates_parsing_error_given_an_invalid_decimallike_string() {
        let decimal_str = Value::test_string("11.6anra");

        let actual = action(&decimal_str, Span::unknown());

        assert_eq!(actual.get_type(), Error);
    }
}
