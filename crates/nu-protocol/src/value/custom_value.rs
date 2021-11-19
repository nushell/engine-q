use std::{cmp::Ordering, fmt};

use crate::{ShellError, Span, Value};

// Trait definition for a custom value
#[typetag::serde(tag = "type")]
pub trait CustomValue: fmt::Debug + Send + Sync {
    fn clone_value(&self, span: Span) -> Value;

    // Define string representation of the custom value
    fn value_string(&self) -> String;

    // Json representation of custom value
    fn to_json(&self) -> nu_json::Value {
        nu_json::Value::Null
    }

    // Any representation used to downcast object to its original type
    fn as_any(&self) -> &dyn std::any::Any;

    // Partial comparison between custom object and a value
    fn partial_cmp(&self, _rhs: &Value) -> Option<Ordering> {
        None
    }

    // Operation definitions for the custom value
    fn add(&self, span: &Span, rhs: &Value) -> Result<Value, ShellError>;
    fn sub(&self, span: &Span, rhs: &Value) -> Result<Value, ShellError>;
    fn mul(&self, span: &Span, rhs: &Value) -> Result<Value, ShellError>;
    fn div(&self, span: &Span, rhs: &Value) -> Result<Value, ShellError>;
}
