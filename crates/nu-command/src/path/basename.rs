use std::path::Path;

use nu_engine::CallExt;
use nu_protocol::{
    engine::Command, Example, ShellError, Signature, Span, Spanned, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "path basename"
    }

    fn signature(&self) -> Signature {
        Signature::build("path basename")
            .named(
                "columns",
                SyntaxShape::Table,
                "Optionally operate by column path",
                Some('c'),
            )
            .named(
                "replace",
                SyntaxShape::String,
                "Return original path with basename replaced by this string",
                Some('r'),
            )
    }

    fn usage(&self) -> &str {
        "Get the final component of a path"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let head = call.head;
        let columns: Option<Vec<String>> = call.get_flag(engine_state, stack, "columns")?;
        let replace: Option<Spanned<String>> = call.get_flag(engine_state, stack, "replace")?;

        input.map(
            move |value| operate(value, head, columns.clone(), replace.clone()),
            engine_state.ctrlc.clone(),
        )
    }

    #[cfg(windows)]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Get basename of a path",
                example: "'C:\\Users\\joe\\test.txt' | path basename",
                result: Some(Value::test_string("test.txt")),
            },
            Example {
                description: "Get basename of a path in a column",
                example: "ls .. | path basename -c [ name ]",
                result: None,
            },
            Example {
                description: "Get basename of a path in a column",
                example: "[[name];[C:\\Users\\Joe]] | path basename -c [ name ]",
                result: Some(Value::test_string("joe")),
            },
            Example {
                description: "Replace basename of a path",
                example: "'C:\\Users\\joe\\test.txt' | path basename -r 'spam.png'",
                result: Some(Value::test_string("/home/joe/spam.png")),
            },
        ]
    }

    #[cfg(not(windows))]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Get basename of a path",
                example: "'/home/joe/test.txt' | path basename",
                result: Some(Value::test_string("test.txt")),
            },
            Example {
                description: "Get basename of a path by column",
                example: "[name];[/home/joe] | path basename -c [ name ]",
                result: Some(Value::List {
                    vals: vec![Value::test_string("joe")],
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "Replace basename of a path",
                example: "'/home/joe/test.txt' | path basename -r 'spam.png'",
                result: Some(Value::test_string("/home/joe/spam.png")),
            },
        ]
    }
}

fn operate(
    v: Value,
    name: Span,
    columns: Option<Vec<String>>,
    replace: Option<Spanned<String>>,
) -> Value {
    match v {
        Value::String { val, span } => get_basename(val, replace.clone(), span),
        Value::Record { cols, vals, span } => {
            let col = if let Some(col) = columns { col } else { vec![] };
            if col.len() == 0 {
                return Value::Error {
                    error: ShellError::UnsupportedInput(
                        String::from("when the input is a table, you must specify the columns"),
                        name,
                    ),
                };
            }

            let mut output_cols = vec![];
            let mut output_vals = vec![];

            for (k, v) in cols.iter().zip(vals) {
                output_cols.push(k.clone());
                if col.contains(k) {
                    let new_val = match v {
                        Value::String { val, span } => get_basename(val, replace.clone(), span),
                        _ => return handle_invalid_values(v, name),
                    };
                    output_vals.push(new_val);
                } else {
                    output_vals.push(v);
                }
            }

            Value::Record {
                cols: output_cols,
                vals: output_vals,
                span,
            }
        }
        _ => handle_invalid_values(v, name),
    }
}

fn handle_invalid_values(rest: Value, name: Span) -> Value {
    Value::Error {
        error: match rest.span() {
            Ok(span) => ShellError::PipelineMismatch {
                expected: Type::String,
                expected_span: name,
                origin: span,
            },
            Err(error) => error,
        },
    }
}

fn get_basename(val: String, replace: Option<Spanned<String>>, span: Span) -> Value {
    let path = Path::new(&val);
    match replace {
        Some(r) => Value::string(path.with_file_name(r.item).to_string_lossy(), span),
        None => Value::string(
            match path.file_name() {
                Some(n) => n.to_string_lossy(),
                None => "".into(),
            },
            span,
        ),
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
