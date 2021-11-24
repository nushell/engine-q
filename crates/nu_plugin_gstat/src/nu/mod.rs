use crate::GStat;
use nu_plugin::{plugin::PluginError, Plugin};
use nu_protocol::ast::Call;
use nu_protocol::{Signature, Span, SyntaxShape, Value};

impl Plugin for GStat {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("gstat")
            .desc("Get the git status of a repo")
            .optional("path", SyntaxShape::String, "path to repo")]
    }

    fn run(&mut self, name: &str, call: &Call, input: &Value) -> Result<Value, PluginError> {
        // eprintln!(
        //     "mod_run: name: {} call: {:?} input: {:?}",
        //     name, &call, input
        // );
        if name != "gstat" {
            return Ok(Value::Nothing {
                span: Span::unknown(),
            });
        }

        // let path_exp = call.get_flag_expr("path");
        // eprintln!("path_exp: {:?}", path_exp);
        // let has_path = call.has_flag("path");
        // eprintln!("has_path: {:?}", has_path);
        // // let zero = call.nth(0)?;
        let path: Option<String> = Some("blah".to_string());

        // if call.has_flag("major") {
        //     self.for_semver(SemVerAction::Major);
        // }
        // if call.has_flag("minor") {
        //     self.for_semver(SemVerAction::Minor);
        // }
        // if call.has_flag("patch") {
        //     self.for_semver(SemVerAction::Patch);
        // }

        // input only appears to be valid if you do echo "blah" | gstat

        self.gstat(input, path)
    }
}
