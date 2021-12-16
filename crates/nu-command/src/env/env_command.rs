use nu_engine::env_to_string;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, IntoPipelineData, PipelineData, Signature, Value};

#[derive(Clone)]
pub struct Env;

impl Command for Env {
    fn name(&self) -> &str {
        "env"
    }

    fn usage(&self) -> &str {
        "Display current environment"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("env").category(Category::Env)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let span = call.head;
        let config = stack.get_config().unwrap_or_default();

        let mut values = vec![];

        for (name, val) in stack.get_env_vars().into_iter() {
            let mut cols = vec![];
            let mut vals = vec![];

            let raw = env_to_string(&name, val.clone(), engine_state, stack, &config)?;

            cols.push("name".into());
            vals.push(Value::string(name, span));

            cols.push("value".into());
            vals.push(val);

            cols.push("raw".into());
            vals.push(Value::string(raw, span));

            values.push(Value::Record { cols, vals, span });
        }

        Ok(Value::List { vals: values, span }.into_pipeline_data())
    }
}
