use std::cmp::Ordering;

use crate::NuDataFrame;
use nu_protocol::{CustomValue, ShellError, Span, Type, Value};

// CustomValue implementation for NuDataFrame
impl CustomValue for NuDataFrame {
    fn typetag_name(&self) -> &'static str {
        "dataframe"
    }

    fn typetag_deserialize(&self) {
        unimplemented!("typetag_deserialize")
    }

    fn clone_value(&self, span: nu_protocol::Span) -> Value {
        let cloned = NuDataFrame(self.0.clone());

        Value::CustomValue {
            val: Box::new(cloned),
            span,
        }
    }

    fn value_string(&self) -> String {
        self.typetag_name().to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let vals = self.print()?;

        Ok(Value::List { vals, span })
    }

    fn to_json(&self) -> nu_json::Value {
        nu_json::Value::Null
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn partial_cmp(&self, _rhs: &Value) -> Option<Ordering> {
        None
    }

    fn follow_path_int(&self, count: usize, span: Span) -> Result<Value, ShellError> {
        self.get_value(count, span)
    }

    fn follow_path_string(&self, column_name: String, span: Span) -> Result<Value, ShellError> {
        let column = self.column(&column_name, span)?;
        Ok(column.to_value(span))
    }

    fn add(&self, span: &Span, _op: Span, rhs: &Value) -> Result<Value, ShellError> {
        match rhs {
            Value::CustomValue {
                val: rhs_val,
                span: rhs_span,
            } => {
                let _rhs_df = rhs_val
                    .as_any()
                    .downcast_ref::<NuDataFrame>()
                    .ok_or_else(|| {
                        ShellError::DowncastNotPossible(
                            "Unable to downcast as Nu-dataframe".into(),
                            *rhs_span,
                        )
                    })?;

                todo!()
            }
            _ => Err(ShellError::OperatorMismatch {
                op_span: Span::unknown(),
                lhs_ty: Type::Custom,
                lhs_span: *span,
                rhs_ty: rhs.get_type(),
                rhs_span: rhs.span()?,
            }),
        }
    }

    fn sub(&self, _span: &Span, _op: Span, _rhs: &Value) -> Result<Value, ShellError> {
        todo!()
    }

    fn mul(&self, _span: &Span, _op: Span, _rhs: &Value) -> Result<Value, ShellError> {
        todo!()
    }

    fn div(&self, _span: &Span, _op: Span, _rhs: &Value) -> Result<Value, ShellError> {
        todo!()
    }
}
