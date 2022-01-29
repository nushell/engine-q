use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, SyntaxShape,
    Value,
};

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "rotate counter-clockwise"
    }

    fn signature(&self) -> Signature {
        Signature::build("rotate counter-clockwise")
            .rest(
                "rest",
                SyntaxShape::String,
                "the names to give columns once rotated",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Rotates the table by -90 degrees (counter clockwise)."
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Rotate table",
                example: "[[a b]; [1 2]] | rotate counter-clockwise",
                result: Some(Value::List {
                    vals: vec![
                        Value::Record {
                            cols: vec!["Column0".to_string(), "Column1".to_string()],
                            vals: vec![Value::test_string("b"), Value::test_int(2)],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec!["Column0".to_string(), "Column1".to_string()],
                            vals: vec![Value::test_string("a"), Value::test_int(1)],
                            span: Span::test_data(),
                        },
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Rotate table",
                example: "[[a b]; [1 2] [3 4] [5 6]] | rotate counter-clockwise",
                result: Some(Value::List {
                    vals: vec![
                        Value::Record {
                            cols: vec![
                                "Column0".to_string(),
                                "Column1".to_string(),
                                "Column2".to_string(),
                                "Column3".to_string(),
                            ],
                            vals: vec![
                                Value::test_string("b"),
                                Value::test_int(2),
                                Value::test_int(4),
                                Value::test_int(6),
                            ],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec![
                                "Column0".to_string(),
                                "Column1".to_string(),
                                "Column2".to_string(),
                                "Column3".to_string(),
                            ],
                            vals: vec![
                                Value::test_string("a"),
                                Value::test_int(1),
                                Value::test_int(3),
                                Value::test_int(5),
                            ],
                            span: Span::test_data(),
                        },
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Rotate table",
                example: "[[a b]; [1 2]] | rotate counter-clockwise aa bb",
                result: Some(Value::List {
                    vals: vec![
                        Value::Record {
                            cols: vec!["aa".to_string(), "bb".to_string()],
                            vals: vec![Value::test_string("b"), Value::test_int(2)],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec!["aa".to_string(), "bb".to_string()],
                            vals: vec![Value::test_string("a"), Value::test_int(1)],
                            span: Span::test_data(),
                        },
                    ],
                    span: Span::test_data(),
                }),
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        rotate(engine_state, stack, call, input)
    }
}

pub fn rotate(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let col_given_names: Vec<String> = call.rest(engine_state, stack, 0)?;
    let values = input.into_iter().collect::<Vec<_>>();
    let mut old_column_names = vec![];
    let mut new_values = vec![];
    let mut not_a_record = false;
    let total_rows = &mut values.len();

    if !values.is_empty() {
        for val in values.into_iter() {
            match val {
                Value::Record {
                    cols,
                    vals,
                    span: _,
                } => {
                    old_column_names = cols;
                    for v in vals {
                        new_values.push(v)
                    }
                }
                Value::List { vals, span: _ } => {
                    not_a_record = true;
                    for v in vals {
                        new_values.push(v);
                    }
                }
                Value::String { val, span } => {
                    not_a_record = true;
                    new_values.push(Value::String { val, span })
                }
                x => {
                    not_a_record = true;
                    new_values.push(x)
                }
            }
        }
    } else {
        return Err(ShellError::UnsupportedInput(
            "Rotate command requires a Nu value as input".to_string(),
            call.head,
        ));
    }

    let total_columns = &old_column_names.len();

    if *total_columns == 0 {
        *total_rows -= 1;
    }
    // holder for the new column names, particularly if none are provided by the user we create names as Column0, Column1, etc.
    let mut new_column_names = {
        let mut res = vec![];
        for idx in 0..(*total_rows + 1) {
            res.push(format!("Column{}", idx));
        }
        res.to_vec()
    };

    // we got new names for columns from the input, so we need to swap those we already made
    if !col_given_names.is_empty() {
        for (idx, val) in col_given_names.into_iter().enumerate() {
            if idx > new_column_names.len() - 1 {
                break;
            }
            new_column_names[idx] = val;
        }
    }

    if not_a_record {
        return Ok(Value::List {
            vals: vec![Value::Record {
                cols: new_column_names,
                vals: new_values,
                span: call.head,
            }],
            span: call.head,
        }
        .into_pipeline_data());
    }

    // holder for the new records
    let mut final_values = vec![];

    // the number of initial columns will be our number of rows, so we iterate through that to get the new number of rows that we need to make
    // as we're rotating counter clockwise, we're iterating from right to left
    for (idx, val) in old_column_names.iter().enumerate().rev() {
        let mut res = vec![Value::String {
            val: val.to_string(),
            span: call.head,
        }];

        let new_vals = {
            // move through the array every 2 elements, starting from our old column's index
            // so if initial data was like this [[a b]; [1 2] [3 4]] - we basically iterate on this [1 2 3 4] array, so we pick 2, then 4, and then when idx decreases (notice the .rev()), we pick 1 and 3
            for i in (idx..new_values.len()).step_by(new_values.len() / *total_rows) {
                res.push(new_values[i].clone());
            }

            res.to_vec()
        };
        final_values.push(Value::Record {
            cols: new_column_names.clone(),
            vals: new_vals,
            span: call.head,
        })
    }

    Ok(Value::List {
        vals: final_values,
        span: call.head,
    }
    .into_pipeline_data())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand)
    }
}
