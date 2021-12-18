use nu_engine::CallExt;
use nu_protocol::{
    ast::{Call, CellPath},
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "into bool"
    }

    fn signature(&self) -> Signature {
        Signature::build("into bool")
            .rest(
                "rest",
                SyntaxShape::CellPath,
                "column paths to convert to binary (for table input)",
            )
            .category(Category::Conversions)
    }

    fn usage(&self) -> &str {
        "Convert value to boolean"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        into_bool(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Convert string to boolean in table",
                example: "echo [[num]; ['false'] ['1'] [0] [1.0]] | into int num",
                result: None,
            },
            Example {
                description: "convert decimal to boolean",
                example: "1 | into bool",
                result: Some(Value::boolean(true, Span::unknown())),
            },
            Example {
                description: "convert decimal string to boolean",
                example: "'0.0' | into bool",
                result: Some(Value::boolean(false, Span::unknown())),
            },
            Example {
                description: "convert string to boolean",
                example: "'true' | into bool",
                result: Some(Value::boolean(true, Span::unknown())),
            },
            Example {
                description: "convert a file exists to boolean",
                example: "ls LICENSE | into bool",
                result: None,
            },
        ]
    }
}

fn into_bool(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let head = call.head;
    let column_paths: Vec<CellPath> = call.rest(engine_state, stack, 0)?;

    input.map(
        move |v| {
            if column_paths.is_empty() {
                action(&v, head)
            } else {
                unimplemented!()
            }
        },
        engine_state.ctrlc.clone(),
    )
}

fn string_to_boolean(s: &str, span: Span) -> Result<bool, ShellError> {
    match s.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        o => {
            let val = o.parse::<f64>();
            match val {
                Ok(f) => Ok(f != 0.0),
                Err(e) => Err(ShellError::CantConvert(
                    "boolean".to_string(),
                    "string".to_string(),
                    span,
                )),
            }
        }
    }
}

fn action(input: &Value, span: Span) -> Value {
    match input {
        Value::Int { val, .. } => Value::Bool {
            val: *val != 0,
            span,
        },
        Value::Float { val, .. } => Value::Bool {
            val: *val != 0.0,
            span,
        },
        Value::String { val, .. } => match string_to_boolean(val, span) {
            Ok(val) => Value::Bool { val, span },
            Err(error) => Value::Error { error },
        },
        _ => Value::Error {
            error: ShellError::UnsupportedInput(
                "'into bool' does not support this input".into(),
                span,
            ),
        },
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
