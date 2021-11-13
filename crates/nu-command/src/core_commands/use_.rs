use nu_engine::eval_block;
use nu_protocol::ast::{Call, Expr, Expression, ImportPatternMember};
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{PipelineData, ShellError, Signature, Span, SyntaxShape};

#[derive(Clone)]
pub struct Use;

impl Command for Use {
    fn name(&self) -> &str {
        "use"
    }

    fn usage(&self) -> &str {
        "Use definitions from a module"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("use").rest("pattern", SyntaxShape::String, "import pattern parts")
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let import_pattern = if let Some(Expression {
            expr: Expr::ImportPattern(pat),
            ..
        }) = call.positional.get(0)
        {
            pat
        } else {
            return Err(ShellError::InternalError(
                "Got something else than import pattern".into(),
            ));
        };

        if let Some(block_id) = engine_state.find_module(&import_pattern.head.name) {
            let overlay = &engine_state.get_block(block_id).overlay;

            let env_vars_to_use = if import_pattern.members.is_empty() {
                overlay.env_vars_with_head(&import_pattern.head.name)
            } else {
                match &import_pattern.members[0] {
                    ImportPatternMember::Glob { .. } => overlay.env_vars(),
                    ImportPatternMember::Name { name, span } => {
                        let mut output = vec![];

                        if let Some(id) = overlay.get_env_var_id(name) {
                            output.push((name.clone(), id));
                        } else if !overlay.has_decl(name) {
                            return Err(ShellError::EnvVarNotFoundAtRuntime(*span));
                        }

                        output
                    }
                    ImportPatternMember::List { names } => {
                        let mut output = vec![];

                        for (name, span) in names {
                            if let Some(id) = overlay.get_env_var_id(name) {
                                output.push((name.clone(), id));
                            } else if !overlay.has_decl(name) {
                                return Err(ShellError::EnvVarNotFoundAtRuntime(*span));
                            }
                        }

                        output
                    }
                }
            };

            for (name, block_id) in env_vars_to_use {
                let name = if let Ok(s) = String::from_utf8(name.clone()) {
                    s
                } else {
                    return Err(ShellError::NonUtf8(import_pattern.head.span));
                };

                let block = engine_state.get_block(block_id);

                // TODO: Later expand env to take all Values
                let val = if let Ok(s) =
                    eval_block(engine_state, stack, block, PipelineData::new(call.head))?
                        .into_value(Span::unknown())
                        .as_string()
                {
                    s
                } else {
                    return Err(ShellError::EnvVarNotAString(import_pattern.span()));
                };

                stack.add_env_var(name, val);
            }
        } else {
            return Err(ShellError::EnvVarNotFoundAtRuntime(call.positional[0].span));
        }

        Ok(PipelineData::new(call.head))
    }
}
