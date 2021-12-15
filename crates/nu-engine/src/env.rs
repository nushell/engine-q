use std::collections::HashMap;

use nu_protocol::engine::{EngineState, Stack};
use nu_protocol::{Config, PipelineData, ShellError, Span};

use crate::eval_block;

/// Translate environment variables from Strings to Values. Requires config to be already set up in
/// case the user defined custom env conversions in config.nu.
pub fn env_to_values(
    engine_state: &EngineState,
    stack: &mut Stack,
    config: &Config,
) -> Option<ShellError> {
    let mut new_env_vars = vec![];
    let mut error = None;

    for scope in &stack.env_vars {
        let mut new_scope = HashMap::new();

        for (name, val) in scope {
            if let Some(conv) = config.env_conversions.get(name) {
                let block = engine_state.get_block(conv.from_string.0);

                if let Some(var) = block.signature.get_positional(0) {
                    let mut stack = stack.collect_captures(&block.captures);
                    if let Some(var_id) = &var.var_id {
                        stack.add_var(*var_id, val.clone());
                    }

                    let result = eval_block(
                        engine_state,
                        &mut stack,
                        block,
                        PipelineData::new(Span::unknown()),
                    );

                    match result {
                        Ok(data) => {
                            let val = data.into_value(Span::unknown());
                            new_scope.insert(name.to_string(), val);
                        }
                        Err(e) => error = error.or(Some(e)),
                    }
                } else {
                    error = error.or_else(|| Some(ShellError::MissingParameter(
                        "block input".into(),
                        conv.from_string.1,
                    )));
                }
            } else {
                new_scope.insert(name.to_string(), val.clone());
            }
        }

        new_env_vars.push(new_scope);
    }

    stack.env_vars = new_env_vars;

    error
}

/// Translate environment variables from Values to Strings
pub fn env_to_strings(
    engine_state: &EngineState,
    stack: &mut Stack,
    config: &Config,
) -> Result<HashMap<String, String>, ShellError> {
    let env_vars = stack.get_env_vars();
    let mut env_vars_str = HashMap::new();
    for (env_name, val) in env_vars {
        if let Some(conv) = config.env_conversions.get(&env_name) {
            let block = engine_state.get_block(conv.to_string.0);

            if let Some(var) = block.signature.get_positional(0) {
                let mut stack = stack.collect_captures(&block.captures);
                if let Some(var_id) = &var.var_id {
                    stack.add_var(*var_id, val);
                }

                let val_str = eval_block(
                    engine_state,
                    &mut stack,
                    block,
                    PipelineData::new(Span::unknown()),
                )?
                .into_value(Span::unknown())
                .as_string()?;

                env_vars_str.insert(env_name, val_str);
            } else {
                return Err(ShellError::MissingParameter(
                    "block input".into(),
                    conv.to_string.1,
                ));
            }
        }
    }

    Ok(env_vars_str)
}
