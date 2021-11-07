use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{IntoInterruptiblePipelineData, PipelineData, ShellError, Signature};

#[derive(Clone)]
pub struct Reverse;

impl Command for Reverse {
    fn name(&self) -> &str {
        "reverse"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("reverse")
    }

    fn usage(&self) -> &str {
        "Reverses the table."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let v: Vec<_> = input.into_iter().collect();
        let iter = v.into_iter().rev();
        Ok(iter.into_pipeline_data(engine_state.ctrlc.clone()))
    }
}
