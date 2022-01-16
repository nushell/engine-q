use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use std::io::Write;

#[derive(Clone)]
pub struct Input;

impl Command for Input {
    fn name(&self) -> &str {
        "input"
    }

    fn usage(&self) -> &str {
        "Get input from the user."
    }

    fn signature(&self) -> Signature {
        Signature::build("input")
            .optional("prompt", SyntaxShape::String, "prompt to show the user")
            .category(Category::Platform)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let prompt: Option<String> = call.opt(engine_state, stack, 0)?;

        if let Some(prompt) = prompt {
            print!("{}", prompt);
            let _ = std::io::stdout().flush();
        }

        let mut buf = String::new();
        let input = std::io::stdin().read_line(&mut buf);

        match input {
            Ok(_) => Ok(Value::String {
                val: buf,
                span: call.head,
            }
            .into_pipeline_data()),
            Err(err) => Err(ShellError::IOError(err.to_string())),
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Get input from the user, and assign to a variable",
            example: "let user-input = (input)",
            result: None,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn examples_work_as_expected() {
        use crate::test_examples;
        test_examples(Input {})
    }
}
