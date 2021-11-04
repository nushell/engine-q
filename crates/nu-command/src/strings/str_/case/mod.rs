pub mod camel_case;
pub mod command;
pub mod kebab_case;
pub mod pascal_case;
// pub mod screaming_snake_case;
// pub mod snake_case;
pub use command::Case;

pub use camel_case::SubCommand as CamelCase;
pub use kebab_case::SubCommand as KebabCase;
pub use pascal_case::SubCommand as PascalCase;

use nu_protocol::{ShellError, Span, Value};
// pub use screaming_snake_case::SubCommand as ScreamingSnakeCase;
// pub use snake_case::SubCommand as SnakeCase;

// struct Arguments {
//     column_paths: Vec<ColumnPath>,
// }

// pub fn operate<F>(args: CommandArgs, case_operation: &'static F) -> Result<OutputStream, ShellError>
// where
//     F: Fn(&str) -> String + Send + Sync + 'static,
// {
//     let (options, input) = (
//         Arguments {
//             column_paths: args.rest(0)?,
//         },
//         args.input,
//     );

//     Ok(input
//         .map(move |v| {
//             if options.column_paths.is_empty() {
//                 action(&v, v.tag(), &case_operation)
//             } else {
//                 let mut ret = v;

//                 for path in &options.column_paths {
//                     ret = ret.swap_data_by_column_path(
//                         path,
//                         Box::new(move |old| action(old, old.tag(), &case_operation)),
//                     )?;
//                 }

//                 Ok(ret)
//             }
//         })
//         .into_input_stream())
// }

pub fn action<F>(input: &Value, case_operation: &F) -> Value
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    match input {
        Value::String { val, span } => Value::String {
            val: case_operation(val),
            span: *span,
        },
        other => Value::Error {
            error: ShellError::UnsupportedInput(
                format!(
                    "Input's type is {}. This command only works with strings.",
                    other.get_type()
                ),
                Span::unknown(),
            ),
        },
    }
}
