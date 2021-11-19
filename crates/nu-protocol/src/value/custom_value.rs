use std::fmt;

use crate::{Span, Value};

// Trait definition for a custom value
#[typetag::serde(tag = "type")]
pub trait CustomValue: fmt::Debug + Send + Sync {
    fn clone_value(&self, span: Span) -> Value;
    fn value_string(&self) -> String;

    fn to_json(&self) -> nu_json::Value {
        nu_json::Value::Null
    }

    fn add(&self, rhs: &Value) -> Value;
}
