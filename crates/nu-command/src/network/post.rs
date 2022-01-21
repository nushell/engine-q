use base64::encode;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::ByteStream;

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};
use reqwest::blocking::Response;

use std::io::{BufRead, BufReader, Read};

use reqwest::StatusCode;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "post"
    }

    fn signature(&self) -> Signature {
        Signature::build("post")
            .desc("Post content to a URL and retrieve data as a table if possible.")
            .required("path", SyntaxShape::String, "the URL to post to")
            .required("body", SyntaxShape::Any, "the contents of the post body")
            .named(
                "user",
                SyntaxShape::Any,
                "the username when authenticating",
                Some('u'),
            )
            .named(
                "password",
                SyntaxShape::Any,
                "the password when authenticating",
                Some('p'),
            )
            .named(
                "content-type",
                SyntaxShape::Any,
                "the MIME type of content to post",
                Some('t'),
            )
            .named(
                "content-length",
                SyntaxShape::Any,
                "the length of the content being posted",
                Some('l'),
            )
            .switch(
                "raw",
                "return values as a string instead of a table",
                Some('r'),
            )
            .switch(
                "insecure",
                "allow insecure server connections when using SSL",
                Some('k'),
            )
            .filter()
            .category(Category::Network)
    }
    fn usage(&self) -> &str {
        "Post a body to a URL (HTTP POST operation)."
    }
    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        run_post(engine_state, stack, call, input)
    }
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Post content to url.com",
                example: "post url.com 'body'",
                result: None,
            },
            Example {
                description: "Post content to url.com, with username and password",
                example: "post -u myuser -p mypass url.com 'body'",
                result: None,
            },
        ]
    }
}

struct Arguments {
    path: Option<Value>,
    body: Option<Value>,
    raw: bool,
    insecure: Option<bool>,
    user: Option<String>,
    password: Option<String>,
    content_type: Option<String>,
    content_length: Option<String>,
}
fn run_post(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    _input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let args = Arguments {
        path: Some(call.req(engine_state, stack, 0)?),
        body: Some(call.req(engine_state, stack, 1)?),
        raw: call.has_flag("raw"),
        user: call.get_flag(engine_state, stack, "user")?,
        password: call.get_flag(engine_state, stack, "password")?,
        insecure: call.get_flag(engine_state, stack, "insecure")?,
        content_type: call.get_flag(engine_state, stack, "content_type")?,
        content_length: call.get_flag(engine_state, stack, "content_length")?,
    };
    //helper(engine_state, stack, call, args)
    todo!("port post please")
}

// Only panics if the user agent is invalid but we define it statically so either
// it always or never fails
#[allow(clippy::unwrap_used)]
fn http_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("nushell")
        .build()
        .unwrap()
}
