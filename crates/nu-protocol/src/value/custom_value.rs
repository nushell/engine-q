use std::{cmp::Ordering, fmt};

use crate::{ShellError, Span, Value};

// Trait definition for a custom value
#[typetag::serde(tag = "type")]
pub trait CustomValue: fmt::Debug + Send + Sync {
    fn clone_value(&self, span: Span) -> Value;

    // Define string representation of the custom value
    fn value_string(&self) -> String;

    // Converts the custom value to a base nushell value
    // This is used to represent the custom value using the table representations
    // That already exist in nushell
    fn to_base_value(&self, span: Span) -> Result<Value, ShellError>;

    // Json representation of custom value
    fn to_json(&self) -> nu_json::Value {
        nu_json::Value::Null
    }

    // Any representation used to downcast object to its original type
    fn as_any(&self) -> &dyn std::any::Any;

    // Follow cell path functions
    fn follow_path_int(&self, count: usize, span: Span) -> Result<Value, ShellError>;
    fn follow_path_string(&self, column_name: String, span: Span) -> Result<Value, ShellError>;

    // Partial comparison between custom object and a value
    fn partial_cmp(&self, _rhs: &Value) -> Option<Ordering> {
        None
    }

    // Operation definitions for the custom value
    fn add(&self, span: &Span, op: Span, rhs: &Value) -> Result<Value, ShellError>;
    fn sub(&self, span: &Span, op: Span, rhs: &Value) -> Result<Value, ShellError>;
    fn mul(&self, span: &Span, op: Span, rhs: &Value) -> Result<Value, ShellError>;
    fn div(&self, span: &Span, op: Span, rhs: &Value) -> Result<Value, ShellError>;
}
