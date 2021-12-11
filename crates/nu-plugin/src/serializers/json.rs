use nu_protocol::ShellError;

use crate::{plugin::PluginEncoder, protocol::PluginResponse};

#[derive(Clone)]
pub struct JsonSerializer;

impl PluginEncoder for JsonSerializer {
    fn encode_call(
        &self,
        plugin_call: &crate::protocol::PluginCall,
        writer: &mut impl std::io::Write,
    ) -> Result<(), nu_protocol::ShellError> {
        serde_json::to_writer(writer, plugin_call)
            .map_err(|err| ShellError::PluginFailedToEncode(err.to_string()))
    }

    fn decode_call(
        &self,
        reader: &mut impl std::io::BufRead,
    ) -> Result<crate::protocol::PluginCall, nu_protocol::ShellError> {
        eprintln!("decoding the call");

        let mut buf = Vec::new();
        let a = reader.read_to_end(&mut buf);
        eprintln!("{:?}", a);

        serde_json::from_reader(reader)
            .map_err(|err| ShellError::PluginFailedToEncode(err.to_string()))
    }

    fn encode_response(
        &self,
        plugin_response: &PluginResponse,
        writer: &mut impl std::io::Write,
    ) -> Result<(), ShellError> {
        serde_json::to_writer(writer, plugin_response)
            .map_err(|err| ShellError::PluginFailedToEncode(err.to_string()))
    }

    fn decode_response(
        &self,
        reader: &mut impl std::io::BufRead,
    ) -> Result<PluginResponse, ShellError> {
        serde_json::from_reader(reader)
            .map_err(|err| ShellError::PluginFailedToEncode(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{CallInfo, EvaluatedCall, LabeledError, PluginCall, PluginResponse};
    use nu_protocol::{Signature, Span, Spanned, SyntaxShape, Value};

    #[test]
    fn callinfo_round_trip_signature() {
        let plugin_call = PluginCall::Signature;
        let encoder = JsonSerializer {};

        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_call(&plugin_call, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_call(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginCall::Signature => {}
            PluginCall::CallInfo(_) => panic!("decoded into wrong value"),
        }
    }

    #[test]
    fn callinfo_round_trip_callinfo() {
        let name = "test".to_string();

        let input = Value::Bool {
            val: false,
            span: Span { start: 1, end: 20 },
        };

        let call = EvaluatedCall {
            head: Span { start: 0, end: 10 },
            positional: vec![
                Value::Float {
                    val: 1.0,
                    span: Span { start: 0, end: 10 },
                },
                Value::String {
                    val: "something".into(),
                    span: Span { start: 0, end: 10 },
                },
            ],
            named: vec![(
                Spanned {
                    item: "name".to_string(),
                    span: Span { start: 0, end: 10 },
                },
                Some(Value::Float {
                    val: 1.0,
                    span: Span { start: 0, end: 10 },
                }),
            )],
        };

        let plugin_call = PluginCall::CallInfo(Box::new(CallInfo {
            name: name.clone(),
            call: call.clone(),
            input: input.clone(),
        }));

        let encoder = JsonSerializer {};
        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_call(&plugin_call, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_call(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginCall::Signature => panic!("returned wrong call type"),
            PluginCall::CallInfo(call_info) => {
                assert_eq!(name, call_info.name);
                assert_eq!(input, call_info.input);
                assert_eq!(call.head, call_info.call.head);
                assert_eq!(call.positional.len(), call_info.call.positional.len());

                call.positional
                    .iter()
                    .zip(call_info.call.positional.iter())
                    .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));

                call.named
                    .iter()
                    .zip(call_info.call.named.iter())
                    .for_each(|(lhs, rhs)| {
                        // Comparing the keys
                        assert_eq!(lhs.0.item, rhs.0.item);

                        match (&lhs.1, &rhs.1) {
                            (None, None) => {}
                            (Some(a), Some(b)) => assert_eq!(a, b),
                            _ => panic!("not matching values"),
                        }
                    });
            }
        }
    }

    #[test]
    fn response_round_trip_signature() {
        let signature = Signature::build("nu-plugin")
            .required("first", SyntaxShape::String, "first required")
            .required("second", SyntaxShape::Int, "second required")
            .required_named("first_named", SyntaxShape::String, "first named", Some('f'))
            .required_named(
                "second_named",
                SyntaxShape::String,
                "second named",
                Some('s'),
            )
            .rest("remaining", SyntaxShape::Int, "remaining");

        let response = PluginResponse::Signature(vec![signature.clone()]);

        let encoder = JsonSerializer {};
        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_response(&response, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_response(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginResponse::Error(_) => panic!("returned wrong call type"),
            PluginResponse::Value(_) => panic!("returned wrong call type"),
            PluginResponse::Signature(returned_signature) => {
                assert!(returned_signature.len() == 1);
                assert_eq!(signature.name, returned_signature[0].name);
                assert_eq!(signature.usage, returned_signature[0].usage);
                assert_eq!(signature.extra_usage, returned_signature[0].extra_usage);
                assert_eq!(signature.is_filter, returned_signature[0].is_filter);

                signature
                    .required_positional
                    .iter()
                    .zip(returned_signature[0].required_positional.iter())
                    .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));

                signature
                    .optional_positional
                    .iter()
                    .zip(returned_signature[0].optional_positional.iter())
                    .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));

                signature
                    .named
                    .iter()
                    .zip(returned_signature[0].named.iter())
                    .for_each(|(lhs, rhs)| assert_eq!(lhs, rhs));

                assert_eq!(
                    signature.rest_positional,
                    returned_signature[0].rest_positional,
                );
            }
        }
    }

    #[test]
    fn response_round_trip_value() {
        let value = Value::Int {
            val: 10,
            span: Span { start: 2, end: 30 },
        };

        let response = PluginResponse::Value(Box::new(value.clone()));

        let encoder = JsonSerializer {};
        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_response(&response, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_response(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginResponse::Error(_) => panic!("returned wrong call type"),
            PluginResponse::Signature(_) => panic!("returned wrong call type"),
            PluginResponse::Value(returned_value) => {
                assert_eq!(&value, returned_value.as_ref())
            }
        }
    }

    #[test]
    fn response_round_trip_error() {
        let error = LabeledError {
            label: "label".into(),
            msg: "msg".into(),
            span: Some(Span { start: 2, end: 30 }),
        };
        let response = PluginResponse::Error(error.clone());

        let encoder = JsonSerializer {};
        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_response(&response, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_response(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginResponse::Error(msg) => assert_eq!(error, msg),
            PluginResponse::Signature(_) => panic!("returned wrong call type"),
            PluginResponse::Value(_) => panic!("returned wrong call type"),
        }
    }

    #[test]
    fn response_round_trip_error_none() {
        let error = LabeledError {
            label: "label".into(),
            msg: "msg".into(),
            span: None,
        };
        let response = PluginResponse::Error(error.clone());

        let encoder = JsonSerializer {};
        let mut buffer: Vec<u8> = Vec::new();
        encoder
            .encode_response(&response, &mut buffer)
            .expect("unable to serialize message");
        let returned = encoder
            .decode_response(&mut buffer.as_slice())
            .expect("unable to deserialize message");

        match returned {
            PluginResponse::Error(msg) => assert_eq!(error, msg),
            PluginResponse::Signature(_) => panic!("returned wrong call type"),
            PluginResponse::Value(_) => panic!("returned wrong call type"),
        }
    }
}
