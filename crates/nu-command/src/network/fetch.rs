use base64::encode;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::ByteStream;

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};

use std::io::{BufRead, BufReader, Read};

use reqwest::StatusCode;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "fetch"
    }

    fn signature(&self) -> Signature {
        Signature::build("fetch")
            .desc("Load from a URL into a cell, convert to table if possible (avoid by appending '--raw').")
            .required(
                "URL",
                SyntaxShape::String,
                "the URL to fetch the contents from",
            )
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
            .named("timeout", SyntaxShape::Int, "timeout period in seconds", Some('t'))
            .switch("raw", "fetch contents as text rather than a table", Some('r'))            
            .filter()
            .category(Category::Network)
    }

    fn usage(&self) -> &str {
        "Fetch the contents from a URL (HTTP GET operation)."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        run_fetch(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Fetch content from url.com",
                example: "fetch url.com",
                result: None,
            },
            Example {
                description: "Fetch content from url.com, with username and password",
                example: "fetch -u myuser -p mypass url.com",
                result: None,
            },
        ]
    }
}

struct Arguments {
    url: Option<Value>,
    raw: bool,
    user: Option<String>,
    password: Option<String>,
    timeout: Option<Value>,
}

fn run_fetch(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    _input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let args = Arguments {
        url: Some(call.req(engine_state, stack, 0)?),
        raw: call.has_flag("raw"),
        user: call.get_flag(engine_state, stack, "user")?,
        password: call.get_flag(engine_state, stack, "password")?,
        timeout: call.get_flag(engine_state, stack, "timeout")?,
    };
    helper(engine_state, stack, call, args)
}

// Helper function that actually goes to retrieve the resource from the url given
// The Option<String> return a possible file extension which can be used in AutoConvert commands
fn helper(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    args: Arguments,
) -> std::result::Result<PipelineData, ShellError> {
    let url_value = if let Some(val) = args.url {
        val
    } else {
        return Err(ShellError::UnsupportedInput(
            "Expecting a url as a string but got nothing".to_string(),
            call.head,
        ));
    };

    let span = url_value.span()?;
    let requested_url = url_value.as_string()?;
    let url = match url::Url::parse(&requested_url) {
        Ok(u) => u,
        Err(_e) => {
            return Err(ShellError::UnsupportedInput(
                "Incomplete or incorrect url. Expected a full url, e.g., https://www.example.com"
                    .to_string(),
                span,
            ));
        }
    };
    let user = args.user.clone();
    let password = args.password;
    let timeout = args.timeout;
    let raw = args.raw;
    let login = match (user, password) {
        (Some(user), Some(password)) => Some(encode(&format!("{}:{}", user, password))),
        (Some(user), _) => Some(encode(&format!("{}:", user))),
        _ => None,
    };

    let client = http_client();
    let mut request = client.get(url);

    if let Some(timeout) = timeout {
        let val = timeout.as_i64()?;
        if val.is_negative() || val < 1 {
            return Err(ShellError::UnsupportedInput(
                "Timeout value must be an integer and larger than 0".to_string(),
                timeout.span().unwrap_or_else(|_| Span::new(0, 0)),
            ));
        }

        request = request.timeout(Duration::from_secs(val as u64));
    }

    if let Some(login) = login {
        request = request.header("Authorization", format!("Basic {}", login));
    }

    match request.send() {
        Ok(resp) => {
            // let temp = std::fs::File::create("temp_dwl.txt")?;
            // let mut b = BufWriter::new(temp);
            // let _bytes = resp.copy_to(&mut b);
            // let temp1 = std::fs::File::open("temp_dwl.txt")?;
            // let a = BufReader::new(temp1);

            // TODO I guess we should check if all bytes were written/read...
            match resp.headers().get("content-type") {
                Some(content_type) => {
                    let content_type = content_type.to_str().map_err(|e| {
                        ShellError::LabeledError(
                            e.to_string(),
                            "MIME type were invalid".to_string(),
                        )
                    })?;
                    let content_type = mime::Mime::from_str(content_type).map_err(|_| {
                        ShellError::LabeledError(
                            format!("MIME type unknown: {}", content_type),
                            "given unknown MIME type".to_string(),
                        )
                    })?;
                    let ext = match (content_type.type_(), content_type.subtype()) {
                        (mime::TEXT, mime::PLAIN) => {
                            let path_extension = url::Url::parse(&requested_url)
                                .map_err(|_| {
                                    ShellError::LabeledError(
                                        format!("Cannot parse URL: {}", requested_url),
                                        "cannot parse".to_string(),
                                    )
                                })?
                                .path_segments()
                                .and_then(|segments| segments.last())
                                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                                .and_then(|name| {
                                    PathBuf::from(name)
                                        .extension()
                                        .map(|name| name.to_string_lossy().to_string())
                                });
                            path_extension
                        }
                        _ => Some(content_type.subtype().to_string()),
                    };

                    let buffered_input = BufReader::new(resp);

                    let output = PipelineData::ByteStream(
                        ByteStream {
                            stream: Box::new(BufferedReader {
                                input: buffered_input,
                            }),
                            ctrlc: engine_state.ctrlc.clone(),
                        },
                        span,
                        None,
                    );

                    if raw {
                        return Ok(output);
                    }

                    if let Some(ext) = ext {
                        match engine_state.find_decl(format!("from {}", ext).as_bytes()) {
                            Some(converter_id) => engine_state.get_decl(converter_id).run(
                                engine_state,
                                stack,
                                &Call::new(),
                                output,
                            ),
                            None => Ok(output),
                        }
                    } else {
                        Ok(output)
                    }
                }
                None => {
                    let buffered_input = BufReader::new(resp);

                    let output = PipelineData::ByteStream(
                        ByteStream {
                            stream: Box::new(BufferedReader {
                                input: buffered_input,
                            }),
                            ctrlc: engine_state.ctrlc.clone(),
                        },
                        span,
                        None,
                    );
                    Ok(output)
                }
            }
        }
        Err(e) if e.is_timeout() => Err(ShellError::NetworkFailure(
            format!("Request to {} has timed out", requested_url),
            span,
        )),
        Err(e) if e.is_status() => match e.status() {
            Some(err_code) if err_code == StatusCode::NOT_FOUND => Err(ShellError::NetworkFailure(
                format!("Requested file not found (404): {:?}", requested_url),
                span,
            )),
            Some(err_code) if err_code == StatusCode::MOVED_PERMANENTLY => {
                Err(ShellError::NetworkFailure(
                    format!("Resource moved permanently (301): {:?}", requested_url),
                    span,
                ))
            }
            Some(err_code) if err_code == StatusCode::BAD_REQUEST => {
                Err(ShellError::NetworkFailure(
                    format!("Bad request (400) to {:?}", requested_url),
                    span,
                ))
            }
            Some(err_code) if err_code == StatusCode::FORBIDDEN => Err(ShellError::NetworkFailure(
                format!("Access forbidden (403) to {:?}", requested_url),
                span,
            )),
            _ => Err(ShellError::NetworkFailure(
                format!(
                    "Cannot make request to {:?}. Error is {:?}",
                    requested_url,
                    e.to_string()
                ),
                span,
            )),
        },
        Err(e) => Err(ShellError::NetworkFailure(
            format!(
                "Cannot make request to {:?}. Error is {:?}",
                requested_url,
                e.to_string()
            ),
            span,
        )),
    }
}

pub struct BufferedReader<R: Read> {
    input: BufReader<R>,
}

impl<R: Read> Iterator for BufferedReader<R> {
    type Item = Result<Vec<u8>, ShellError>;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = self.input.fill_buf();
        match buffer {
            Ok(s) => {
                let result = s.to_vec();

                let buffer_len = s.len();

                if buffer_len == 0 {
                    None
                } else {
                    self.input.consume(buffer_len);

                    Some(Ok(result))
                }
            }
            Err(e) => Some(Err(ShellError::IOError(e.to_string()))),
        }
    }
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
