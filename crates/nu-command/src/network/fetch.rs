use base64::encode;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{IntoPipelineData, ByteStream};
use reqwest::blocking::Client;

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Value,
};

use std::io::{BufRead, BufReader, Read, Write, BufWriter};


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

fn run_fetch(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    _input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let mut fetch_helper = Fetch::new();

    fetch_helper.setup(engine_state, call, stack)?;

    // let runtime = tokio::runtime::Runtime::new()?;

    // let path = match fetch_helper.path {
    //     Some(p) => p,
    //     None => {
    //         return Err(ShellError::UnsupportedInput(
    //             "The url must be a string".to_string(),
    //             call.head,
    //         ))
    //     }
    // };

    fetch(
        engine_state,
        call,
        stack,
        fetch_helper.url,
        fetch_helper.has_raw,
        fetch_helper.user.clone(),
        fetch_helper.password,
        fetch_helper.timeout,
    )
}

#[derive(Default)]
pub struct Fetch {
    pub url: Option<Value>,
    pub has_raw: bool,
    pub user: Option<String>,
    pub password: Option<String>,
    pub timeout: Option<Value>,
}

impl Fetch {
    pub fn new() -> Fetch {
        Fetch {
            url: None,
            has_raw: false,
            user: None,
            password: None,
            timeout: None,
        }
    }

    pub fn setup(
        &mut self,
        engine_state: &EngineState,
        call: &Call,
        stack: &mut Stack,
    ) -> Result<(), ShellError> {
        self.url = Some(call.req(engine_state, stack, 0)?);
        self.has_raw = call.has_flag("raw");
        self.user = call.get_flag(engine_state, stack, "user")?;
        self.password = call.get_flag(engine_state, stack, "password")?;
        self.timeout = call.get_flag(engine_state, stack, "timeout")?;
        Ok(())
    }
}

fn fetch(
    engine_state: &EngineState,
    call: &Call,
    stack: &mut Stack,
    url: Option<Value>,
    has_raw: bool,
    user: Option<String>,
    password: Option<String>,
    timeout: Option<Value>,
) -> Result<PipelineData, ShellError> {
    let url_value = if let Some(val) = url {
        val
    } else {
        return Err(ShellError::UnsupportedInput("Expecting a url as a string but got nothing".to_string(), call.head))
    };
    let span = url_value.span()?;
    let url = url_value.as_string()?;
    helper(engine_state, stack, call, &url, span, has_raw, user, password, timeout)

    // if let Err(e) = result {
    //     return Err(e);
    // }
    // let (file_extension, value) = result?;

    // let file_extension = if has_raw {
    //     None
    // } else {
    //     // If the extension could not be determined via mimetype, try to use the path
    //     // extension. Some file types do not declare their mimetypes (such as bson files).
    //     file_extension.or_else(|| url.split('.').last().map(String::from))
    // };

    // if let Some(ext) = file_extension {
    //     match engine_state.find_decl(format!("from {}", ext).as_bytes()) {
    //         Some(converter_id) => engine_state.get_decl(converter_id).run(
    //             engine_state,
    //             stack,
    //             &Call::new(),
    //             value.into_pipeline_data(),
    //         ),
    //         None => Ok(value.into_pipeline_data()),
    //     }
    // } else {
    //     Ok(value.into_pipeline_data())
    // }
}

// Helper function that actually goes to retrieve the resource from the url given
// The Option<String> return a possible file extension which can be used in AutoConvert commands
fn helper(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    location: &str,
    span: Span,
    has_raw: bool,
    user: Option<String>,
    password: Option<String>,
    timeout: Option<Value>,
) -> std::result::Result<PipelineData, ShellError> {

    

    let url = match url::Url::parse(location) {
        Ok(u) => u,
        Err(e) => {
            return Err(ShellError::UnsupportedInput(
                "Incomplete or incorrect url. Expected a full url, e.g., https://www.example.com".to_string(),
                span
                
            ));
        }
    };
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
                timeout.span().unwrap_or(Span::new(0, 0)),
            ));
        }
            
        request = request.timeout(Duration::from_secs(val as u64));
    }

    if let Some(login) = login {
        request = request.header("Authorization", format!("Basic {}", login));
    }

    let generate_error = |t: &str, e: reqwest::Error| {
        ShellError::LabeledError(
            format!("Could not load {} from remote url: {:?}", t, e),
            "could not load".to_string(),
        )
    };

    match request.send() {
        Ok(mut resp) => {
          
            
            let temp = std::fs::File::create("temp_dwl.txt")?;
            let mut b = BufWriter::new(temp);
            let bytes = resp.copy_to(&mut b);
            let temp1 = std::fs::File::open("temp_dwl.txt")?;
            let a =  BufReader::new(temp1);

            // let mut buf: Vec<u8> = vec![];
            // let bytes = resp.copy_to(&mut buf);
            // let a =  BufReader::new(buf);

            let output = PipelineData::ByteStream(
                ByteStream {
                    stream: Box::new(BufferedReader { input: a }),
                    
                    ctrlc: engine_state.ctrlc.clone(),
                },
                span,
                None,
            );

            match resp.headers().get("content-type") {
                Some(content_type) => {
                    println!("{:?}", content_type);
                    let content_type = content_type.to_str().map_err(|e| {
                        ShellError::LabeledError(e.to_string(), "MIME type were invalid".to_string())
                    })?;
                    println!("{:?}", content_type);
                    let content_type = mime::Mime::from_str(content_type).map_err(|_| {
                        ShellError::LabeledError(
                            format!("MIME type unknown: {}", content_type),
                            "given unknown MIME type".to_string(),
                        )
                    })?;
                    println!("{:?}", content_type);
                    let ext = Some(content_type.subtype().as_str());
                    println!("{:?}", ext);
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
                }, 

                None => {
                    Ok(output)
                }
  
            // Ok((Some("a".to_string()), Value::test_string("a")))
            }
        },
        Err(e) => Err(ShellError::NetworkFailure(format!("Cannot make the request to {}", location), span))
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
//     match request.send().await {
//         Ok(r) => match r.headers().get("content-type") {
//             Some(content_type) => {
//                 let content_type = content_type.to_str().map_err(|e| {
//                     ShellError::LabeledError(e.to_string(), "MIME type were invalid".to_string())
//                 })?;
//                 let content_type = mime::Mime::from_str(content_type).map_err(|_| {
//                     ShellError::LabeledError(
//                         format!("MIME type unknown: {}", content_type),
//                         "given unknown MIME type".to_string(),
//                     )
//                 })?;
//                 let ext = content_type.subtype().as_str();
//                 let can_convert = match engine_state.find_decl(format!("from {}", ext).as_bytes()) {
//                     Some(id) => Some(id),
//                     None => None
//                 };

//                 let mut buf: Vec<u8> = vec![];

//                 match can_convert {
//                     Some(id) => {
//                         let value = 
//                         Ok((Some(ext), engine_state.get_decl(id).run(
//                             engine_state,
//                             stack,
//                             &Call::new(),
//                             value.into_pipeline_data()),
//                         ))
//                     },
//                     None => 
//                 {
//                     match (content_type.type_(), content_type.subtype()) {
//                     (mime::APPLICATION, mime::XML) => {
//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;
//                         Ok((Some("xml".to_string()), Value::String { val: output, span }))
//                     }
//                     (mime::APPLICATION, mime::JSON) => {
//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;
//                         Ok((
//                             Some("json".to_string()),
//                             Value::String { val: output, span },
//                         ))
//                     }
//                     (mime::APPLICATION, mime::OCTET_STREAM) => {
//                         let buf: Vec<u8> = r
//                             .bytes()
//                             .await
//                             .map_err(|e| generate_error("binary", e))?
//                             .to_vec();
//                         Ok((None, Value::Binary { val: buf, span }))
//                     }
//                     (mime::IMAGE, mime::SVG) => {
//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;
//                         Ok((Some("svg".to_string()), Value::String { val: output, span }))
//                     }
//                     (mime::IMAGE, image_ty) => {
//                         let buf: Vec<u8> = r
//                             .bytes()
//                             .await
//                             .map_err(|e| generate_error("image", e))?
//                             .to_vec();
//                         Ok((Some(image_ty.to_string()), Value::Binary { val: buf, span }))
//                     }
//                     (mime::TEXT, mime::HTML) => {
//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;
//                         Ok((
//                             Some("html".to_string()),
//                             Value::String { val: output, span },
//                         ))
//                     }
//                     (mime::TEXT, mime::CSV) => {
//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;
//                         Ok((Some("csv".to_string()), Value::String { val: output, span }))
//                     }
//                     (mime::TEXT, mime::PLAIN) => {
//                         let path_extension = url::Url::parse(location)
//                             .map_err(|_| {
//                                 ShellError::LabeledError(
//                                     format!("Cannot parse URL: {}", location),
//                                     "cannot parse".to_string(),
//                                 )
//                             })?
//                             .path_segments()
//                             .and_then(|segments| segments.last())
//                             .and_then(|name| if name.is_empty() { None } else { Some(name) })
//                             .and_then(|name| {
//                                 PathBuf::from(name)
//                                     .extension()
//                                     .map(|name| name.to_string_lossy().to_string())
//                             });

//                         let output = r.text().await.map_err(|e| generate_error("text", e))?;

//                         Ok((path_extension, Value::String { val: output, span }))
//                     }
//                     (_ty, _sub_ty) if has_raw => {
//                         let raw_bytes = r.bytes().await;
//                         let raw_bytes = match raw_bytes {
//                             Ok(r) => r,
//                             Err(e) => {
//                                 return Err(ShellError::LabeledError(
//                                     "error with raw_bytes".to_string(),
//                                     e.to_string(),
//                                 ));
//                             }
//                         };

//                         // For unsupported MIME types, we do not know if the data is UTF-8,
//                         // so we get the raw body bytes and try to convert to UTF-8 if possible.
//                         match std::str::from_utf8(&raw_bytes) {
//                             Ok(response_str) => Ok((
//                                 None,
//                                 Value::String {
//                                     val: response_str.to_string(),
//                                     span,
//                                 },
//                             )),
//                             Err(_) => Ok((
//                                 None,
//                                 Value::Binary {
//                                     val: raw_bytes.to_vec(),
//                                     span,
//                                 },
//                             )),
//                         }
//                     }
//                     (ty, sub_ty) => Err(ShellError::UnsupportedInput(
//                         format!("Not yet supported MIME type: {} {}", ty, sub_ty),
//                         span,
//                     )),
//                 }
//                 }
//             }
//             // TODO: Should this return "nothing" or Err?
//             None => Ok((
//                 None,
//                 Value::String {
//                     val: "No content type found".to_string(),
//                     span,
//                 },
//             )),
//         },
//         Err(_e) => Err(ShellError::NetworkFailure(
//             format!("Cannot connect to {}", location),span
//         )),
//     }
// }

// Only panics if the user agent is invalid but we define it statically so either
// it always or never fails
#[allow(clippy::unwrap_used)]
fn http_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("nushell")
        .build()
        .unwrap()
}
