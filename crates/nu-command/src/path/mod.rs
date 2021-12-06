mod basename;
pub mod command;
mod dirname;
mod exists;
mod expand;
mod r#type;

pub use basename::SubCommand as PathBasename;
pub use command::PathCommand as Path;
pub use dirname::SubCommand as PathDirname;
pub use exists::SubCommand as PathExists;
pub use expand::SubCommand as PathExpand;
pub use r#type::SubCommand as PathType;
use nu_protocol::{ShellError, Span, Type, Value};

trait PathSubcommandArguments {
    fn get_columns(&self) -> Option<Vec<String>>;
}

fn operate<F, A>(cmd: &F, args: &A, v: Value, name: Span) -> Value
where
    F: Fn(String, Span, &A) -> Value + Send + Sync + 'static,
    A: PathSubcommandArguments + Send + Sync + 'static,
{
    match v {
        Value::String { val, span } => cmd(val, span, &args),
        Value::Record { cols, vals, span } => {
            let col = if let Some(col) = args.get_columns() {
                col
            } else {
                vec![]
            };
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
                        Value::String { val, span } => cmd(val, span, &args),
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
