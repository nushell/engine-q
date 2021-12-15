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
        unimplemented!();
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
