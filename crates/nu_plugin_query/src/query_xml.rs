use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Spanned, Value};

#[derive(Default)]
pub struct QueryXml;

impl QueryXml {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn usage() -> &'static str {
        "Usage: query xml"
    }

    pub fn query_xml(
        &self,
        name: &str,
        call: &EvaluatedCall,
        value: &Value,
        path: Option<Spanned<String>>,
    ) -> Result<Value, LabeledError> {
        // // use std::any::Any;
        // // eprintln!("input type: {:?} value: {:#?}", &value.type_id(), &value);
        // // eprintln!("path type: {:?} value: {:#?}", &path.type_id(), &path);

        // // This is a flag to let us know if we're using the input value (value)
        // // or using the path specified (path)
        // let mut using_input_value = false;

        // // let's get the input value as a string
        // let piped_value = match value.as_string() {
        //     Ok(s) => {
        //         using_input_value = true;
        //         s
        //     }
        //     _ => String::new(),
        // };

        // // now let's get the path string
        // let mut a_path = match path {
        //     Some(p) => {
        //         // should we check for input and path? nah.
        //         using_input_value = false;
        //         p
        //     }
        //     None => Spanned {
        //         item: ".".to_string(),
        //         span: *span,
        //     },
        // };

        // // If there was no path specified and there is a piped in value, let's use the piped in value
        // if a_path.item == "." && piped_value.chars().count() > 0 {
        //     a_path.item = piped_value;
        // }

        Ok(Value::string("Hello from query xml2", call.head))
    }
}
