use crate::GStat;
use nu_plugin::{EvaluatedCall, Plugin};
use nu_protocol::{ShellError, Signature, Span, SyntaxShape, Value};

impl Plugin for GStat {
    fn signature(&self) -> Vec<Signature> {
        vec![
            Signature::build("gstat")
                .desc("Get the git status of a repo")
                .optional("path", SyntaxShape::String, "path to repo"), // .named("path", SyntaxShape::String, "path to repo", Some('p'))
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, ShellError> {
        if name != "gstat" {
            return Ok(Value::Nothing {
                span: Span::unknown(),
            });
        }

        let repo_path: Option<String> = call.opt(0)?;

        self.gstat(input, repo_path, &call.head)
    }
}
