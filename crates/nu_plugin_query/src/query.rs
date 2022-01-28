use crate::query_json::execute_json_query;
use crate::query_web::parse_selector_params;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Spanned, Value};

#[derive(Default)]
pub struct Query;

impl Query {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn usage() -> &'static str {
        "Usage: query"
    }

    pub fn query(
        &self,
        name: &str,
        call: &EvaluatedCall,
        value: &Value,
        path: Option<Spanned<String>>,
    ) -> Result<Value, LabeledError> {
        Ok(Value::string("Hello from query", call.head))
    }

    pub fn query_json(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        query: Option<Spanned<String>>,
    ) -> Result<Value, LabeledError> {
        execute_json_query(name, call, input, query)
    }
    pub fn query_web(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        rest: Option<Spanned<String>>,
    ) -> Result<Value, LabeledError> {
        // Ok(Value::string("Hello from query_web1", call.head))
        parse_selector_params(name, call, input, rest)
    }
    pub fn query_xml(
        &self,
        name: &str,
        call: &EvaluatedCall,
        value: &Value,
        path: Option<Spanned<String>>,
    ) -> Result<Value, LabeledError> {
        Ok(Value::string("Hello from query_xml1", call.head))
    }
}
