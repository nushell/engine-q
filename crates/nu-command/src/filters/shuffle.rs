// this used to be last
// delete this comment prior to checkin

//use nu_engine::CallExt;

use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{IntoInterruptiblePipelineData, PipelineData, ShellError, Signature};
//use std::convert::TryInto;

//use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Shuffle;

impl Command for Shuffle {
    fn name(&self) -> &str {
        "shuffle"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("shuffle")
    }

    fn usage(&self) -> &str {
        "Shuffle rows randomly."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let v: Vec<_> = input.into_iter().collect();

        let iter = v.into_iter();

        /*
                let iter = v
                    .into_iter()
                    .shuffle(&mut thread_rng());
        */
        Ok(iter.into_pipeline_data(engine_state.ctrlc.clone()))
    }
}
