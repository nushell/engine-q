use nu_engine::CallExt;
use nu_protocol::{
    ast::Call, engine::Command, engine::EngineState, engine::Stack, Category, Example,
    PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Compact;

impl Command for Compact {
    fn name(&self) -> &str {
        "compact"
    }

    fn signature(&self) -> Signature {
        Signature::build("compact")
            .rest(
                "columns",
                SyntaxShape::Any,
                "the columns to compact from the table",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Creates a table with non-empty rows."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        compact(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Filter out all records where 'Hello' is null (returns nothing)",
            example: r#""[{"Hello": null,"World": 1}]" | from json | compact Hello"#,
            result: Some(Value::List {
                vals: vec![],
                span: Span { start: 0, end: 0 },
            }),
        },
        Example {
            description: "Filter out all records where 'World' is null (Returns the table)",
            example: r#""[{"Hello": null,"World": 1}]" | from json | compact Hello"#,
            result: Some(Value::List {
                vals: vec![],
                span: Span { start: 0, end: 0 },
            }),
        ]
    }
}

pub fn compact(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, ShellError> {
    let columns: Vec<String> = call.rest(engine_state, stack, 0)?;
    println!("Columns {:?}", columns);
    input.filter(
        move |item| {
            println!("{:?}", item);
            match item {
                // Nothing is filtered out
                Value::Nothing { .. } => false,
                Value::Record { .. } => {
                    for column in columns.iter() {
                        println!("got data: {:?}", item.get_data_by_key(column));
                        match item.get_data_by_key(column) {
                            None => return false,
                            Some(x) => match x {
                                Value::Nothing { .. } => return false,
                                _ => (),
                            },
                        }
                    }
                    // No defined columns contained Nothing
                    true
                }
                // Any non-Nothing, non-record should be kept
                _ => true,
            }
        },
        engine_state.ctrlc.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::Compact;

    #[test]
    fn examples_work_as_expected() {
        use crate::test_examples;
        test_examples(Compact {})
    }
}
