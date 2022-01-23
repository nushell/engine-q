use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, IntoPipelineData, PipelineData, ShellError,
    Signature, Span, SyntaxShape, Value,
};

enum BeforeOrAfter {
    Before(String),
    After(String),
}

#[derive(Clone)]
pub struct Move;

impl Command for Move {
    fn name(&self) -> &str {
        "move"
    }

    fn usage(&self) -> &str {
        "Move columns before or after other columns"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("move")
            .rest("columns", SyntaxShape::String, "the columns to move")
            .named(
                "after",
                SyntaxShape::String,
                "the column that will precede the columns moved",
                Some('a'),
            )
            .named(
                "before",
                SyntaxShape::String,
                "the column that will be the next after the columns moved",
                Some('b'),
            )
            .category(Category::Filters)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "",
                description: "Move a column before the first column",
                result: None,
                // Some(Value::List {
                //     vals: vec![
                //         Value::test_record(
                //             vec!["name", "index"],
                //             vec![Value::test_string("a"), Value::test_int(1)],
                //         ),
                //         Value::test_record(
                //             vec!["name", "index"],
                //             vec![Value::test_string("b"), Value::test_int(2)],
                //         ),
                //         Value::test_record(
                //             vec!["name", "index"],
                //             vec![Value::test_string("c"), Value::test_int(3)],
                //         ),
                //     ],
                //     span: Span::test_data(),
                // }),
            },
            Example {
                example: "",
                description: "Move multiple columns after a column",
                result: None,
            },
            Example {
                example: "",
                description: "Move columns of a record to the end",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let columns: Vec<String> = call.rest(engine_state, stack, 0)?;
        let after: Option<String> = call.get_flag(engine_state, stack, "after")?;
        let before: Option<String> = call.get_flag(engine_state, stack, "before")?;

        let before_or_after = match (after, before) {
            (Some(s), None) => BeforeOrAfter::After(s),
            (None, Some(s)) => BeforeOrAfter::Before(s),
            (Some(_), Some(_)) => {
                return Err(ShellError::SpannedLabeledError(
                    "Cannot move columns".to_string(),
                    "Use either --after, or --before, not both".to_string(),
                    call.head,
                ))
            }
            (None, None) => {
                return Err(ShellError::SpannedLabeledError(
                    "Cannot move columns".to_string(),
                    "Missing --after or --before flag".to_string(),
                    call.head,
                ))
            }
        };

        let metadata = input.metadata();
        let ctrlc = engine_state.ctrlc.clone();
        let call = call.clone();

        match input {
            PipelineData::Value(Value::List { .. }, ..) | PipelineData::ListStream { .. } => {
                let res = input.into_iter().map(move |x| match x.as_record() {
                    Ok((inp_cols, inp_vals)) => match move_record_columns(
                        inp_cols,
                        inp_vals,
                        &columns,
                        &before_or_after,
                        call.head,
                    ) {
                        Ok(val) => val,
                        Err(error) => Value::Error { error },
                    },
                    Err(error) => Value::Error { error },
                });

                if let Some(md) = metadata {
                    Ok(res.into_pipeline_data_with_metadata(md, ctrlc))
                } else {
                    Ok(res.into_pipeline_data(ctrlc))
                }
            }
            PipelineData::Value(
                Value::Record {
                    cols: inp_cols,
                    vals: inp_vals,
                    ..
                },
                ..,
            ) => Ok(move_record_columns(
                &inp_cols,
                &inp_vals,
                &columns,
                &before_or_after,
                call.head,
            )?
            .into_pipeline_data()),
            _ => Err(ShellError::PipelineMismatch(
                "record or table".to_string(),
                call.head,
                Span::new(call.head.start, call.head.start),
            )),
        }
    }
}

fn move_record_columns(
    inp_cols: &[String],
    inp_vals: &[Value],
    columns: &[String],
    before_or_after: &BeforeOrAfter,
    span: Span,
) -> Result<Value, ShellError> {
    let mut column_idx: Vec<usize> = Vec::with_capacity(columns.len());

    // Find indices of columns to be moved
    for column in columns.iter() {
        if let Some(idx) = inp_cols.iter().position(|inp_col| column == inp_col) {
            column_idx.push(idx);
        }
    }

    let mut out_cols: Vec<String> = Vec::with_capacity(inp_cols.len());
    let mut out_vals: Vec<Value> = Vec::with_capacity(inp_vals.len());

    for (i, (inp_col, inp_val)) in inp_cols.iter().zip(inp_vals).enumerate() {
        match before_or_after {
            BeforeOrAfter::After(after) if after == inp_col => {
                out_cols.push(inp_col.into());
                out_vals.push(inp_val.clone());

                for idx in column_idx.iter() {
                    if let (Some(col), Some(val)) = (inp_cols.get(*idx), inp_vals.get(*idx)) {
                        out_cols.push(col.into());
                        out_vals.push(val.clone());
                    } else {
                        return Err(ShellError::NushellFailedSpanned(
                            "Error indexing input columns".to_string(),
                            "originates from here".to_string(),
                            span,
                        ));
                    }
                }
            }
            BeforeOrAfter::Before(before) if before == inp_col => {
                for idx in column_idx.iter() {
                    if let (Some(col), Some(val)) = (inp_cols.get(*idx), inp_vals.get(*idx)) {
                        out_cols.push(col.into());
                        out_vals.push(val.clone());
                    } else {
                        return Err(ShellError::NushellFailedSpanned(
                            "Error indexing input columns".to_string(),
                            "originates from here".to_string(),
                            span,
                        ));
                    }
                }

                out_cols.push(inp_col.into());
                out_vals.push(inp_val.clone());
            }
            _ => {
                if !column_idx.contains(&i) {
                    out_cols.push(inp_col.into());
                    out_vals.push(inp_val.clone());
                }
            }
        }
    }

    Ok(Value::Record {
        cols: out_cols,
        vals: out_vals,
        span,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Move {})
    }
}
