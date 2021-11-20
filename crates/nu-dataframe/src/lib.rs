use nu_protocol::{CustomValue, ShellError, Span, Type, Value};
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NuDataFrame(DataFrame);

impl NuDataFrame {
    pub fn new(dataframe: DataFrame) -> Self {
        Self(dataframe)
    }

    pub fn dataframe_to_value(dataframe: DataFrame, span: Span) -> Value {
        Value::CustomValue {
            val: Box::new(Self::new(dataframe)),
            span,
        }
    }
}

impl CustomValue for NuDataFrame {
    fn typetag_name(&self) -> &'static str {
        "nu-dataframe"
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

    fn to_json(&self) -> nu_json::Value {
        nu_json::Value::Null
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
