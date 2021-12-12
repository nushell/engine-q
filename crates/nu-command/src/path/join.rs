use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use nu_engine::CallExt;
use nu_protocol::{
    engine::Command, Example, PipelineData, ShellError, Signature, Span, Spanned, SyntaxShape,
    Value, ValueStream,
};

use super::PathSubcommandArguments;

struct Arguments {
    columns: Option<Vec<String>>,
    append: Option<Spanned<String>>,
}

impl PathSubcommandArguments for Arguments {
    fn get_columns(&self) -> Option<Vec<String>> {
        self.columns.clone()
    }
}

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "path join"
    }

    fn signature(&self) -> Signature {
        Signature::build("path join")
            .named(
                "columns",
                SyntaxShape::Table,
                "Optionally operate by column path",
                Some('c'),
            )
            .optional(
                "append",
                SyntaxShape::Filepath,
                "Path to append to the input",
            )
    }

    fn usage(&self) -> &str {
        "Join a structured path or a list of path parts."
    }

    fn extra_usage(&self) -> &str {
        r#"Optionally, append an additional path to the result. It is designed to accept
the output of 'path parse' and 'path split' subcommands."#
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let args = Arguments {
            columns: call.get_flag(engine_state, stack, "columns")?,
            append: call.opt(engine_state, stack, 0)?,
        };

        match input {
            PipelineData::Value(val, md) => {
                Ok(PipelineData::Value(handle_value(val, &args, head), md))
            }
            PipelineData::Stream(stream, md) => Ok(PipelineData::Stream(
                ValueStream::from_stream(
                    stream.map(move |val| handle_value(val, &args, head)),
                    engine_state.ctrlc.clone(),
                ),
                md,
            )),
        }
    }

    #[cfg(windows)]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Append a filename to a path",
                example: r"'C:\Users\viking' | path join spam.txt",
                result: Some(Value::test_string(r"C:\Users\viking\spam.txt")),
            },
            Example {
                description: "Append a filename to a path inside a column",
                example: r"ls | path join spam.txt -c [ name ]",
                result: None,
            },
            Example {
                description: "Join a list of parts into a path",
                example: r"[ 'C:' '\' 'Users' 'viking' 'spam.txt' ] | path join",
                result: Some(Value::test_string(r"C:\Users\viking\spam.txt")),
            },
            Example {
                description: "Join a structured path into a path",
                example: r"[ [parent stem extension]; ['C:\Users\viking' 'spam' 'txt']] | path join",
                result: Some(Value::test_string(r"C:\Users\viking\spam.txt")),
            },
        ]
    }

    #[cfg(not(windows))]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Append a filename to a path",
                example: r"'/home/viking' | path join spam.txt",
                result: Some(Value::test_string(r"/home/viking/spam.txt")),
            },
            Example {
                description: "Append a filename to a path inside a column",
                example: r"ls | path join spam.txt -c [ name ]",
                result: None,
            },
            Example {
                description: "Join a list of parts into a path",
                example: r"[ '/' 'home' 'viking' 'spam.txt' ] | path join",
                result: Some(Value::test_string(r"/home/viking/spam.txt")),
            },
            Example {
                description: "Join a structured path into a path",
                example: r"[[ parent stem extension ]; [ '/home/viking' 'spam' 'txt' ]] | path join",
                result: Some(Value::test_string(r"/home/viking/spam.txt")),
            },
        ]
    }
}

fn handle_value(v: Value, args: &Arguments, head: Span) -> Value {
    match v {
        Value::String { val, span } => join_single(val, span, args),
        Value::List { vals, span } => match join_list(&vals, span) {
            Ok(v) => Value::string(v.to_string_lossy(), span),
            Err(e) => Value::Error { error: e },
        },

        _ => super::handle_invalid_values(v, head),
    }
}

fn join_single(val: String, span: Span, args: &Arguments) -> Value {
    let path = Path::new(&val);
    let path = if let Some(ref append) = args.append {
        path.join(Path::new(&append.item))
    } else {
        path.to_path_buf()
    };

    Value::string(path.to_string_lossy(), span)
}

fn join_list(parts: &[Value], span: Span) -> Result<PathBuf, ShellError> {
    parts
        .iter()
        .map(|part| match &part {
            Value::String { val, .. } => Ok(Path::new(val).to_path_buf()),
            Value::Record { cols, vals, span } => {
                for key in cols {
                    if !super::ALLOWED_COLUMNS.contains(&&key[..]) {
                        let allowed_cols = super::ALLOWED_COLUMNS.join(", ");
                        let msg = format!("Column '{}' is not valid for a structured path. Allowed columns are: {}", key, allowed_cols);
                        return Err(ShellError::UnsupportedInput(msg, *span));
                    }
                }

                let entries: HashMap<&str, &Value> = cols.iter().map(String::as_str).zip(vals).collect();
                let mut result = PathBuf::new();

                #[cfg(windows)]
                if let Some(val) = entries.get("prefix") {
                    let p = val.as_string()?;
                    if !p.is_empty() {
                        result.push(p);
                    }
                }

                if let Some(val) = entries.get("parent") {
                    let p = val.as_string()?;
                    if !p.is_empty() {
                        result.push(p);
                    }
                }

                let mut basename = String::new();

                if let Some(val) = entries.get("stem") {
                    let p = val.as_string()?;
                    if !p.is_empty() {
                        basename.push_str(&p);
                    }
                }

                if let Some(val) = entries.get("extension") {
                    let p = val.as_string()?;
                    if !p.is_empty() {
                        basename.push('.');
                        basename.push_str(&p);
                    }
                }

                if !basename.is_empty() {
                    result.push(basename);
                }

                Ok(result)
            },

            _ => Err(super::err_from_value(part, span)),
        })
        .collect()
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
