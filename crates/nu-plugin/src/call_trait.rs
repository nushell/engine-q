use nu_protocol::{
    ast::{Call, Expr, Expression},
    FromValue, ShellError, Span, Value,
};

pub trait CallExtPlugin {
    fn get_flag<T: FromValue>(&self, name: &str) -> Result<Option<T>, ShellError>;
    fn rest<T: FromValue>(&self, starting_pos: usize) -> Result<Vec<T>, ShellError>;
    fn opt<T: FromValue>(&self, pos: usize) -> Result<Option<T>, ShellError>;
    fn req<T: FromValue>(&self, pos: usize) -> Result<T, ShellError>;
}

impl CallExtPlugin for Call {
    fn get_flag<T: FromValue>(&self, name: &str) -> Result<Option<T>, ShellError> {
        if let Some(expression) = self.get_flag_expr(name) {
            let value = expression_to_value(expression)?;
            FromValue::from_value(&value).map(Some)
        } else {
            Ok(None)
        }
    }

    fn rest<T: FromValue>(&self, starting_pos: usize) -> Result<Vec<T>, ShellError> {
        self.positional
            .iter()
            .skip(starting_pos)
            .map(|expression| {
                expression_to_value(expression.clone())
                    .and_then(|value| FromValue::from_value(&value))
            })
            .collect()
    }

    fn opt<T: FromValue>(&self, pos: usize) -> Result<Option<T>, ShellError> {
        if let Some(expression) = self.nth(pos) {
            let value = expression_to_value(expression)?;
            FromValue::from_value(&value).map(Some)
        } else {
            Ok(None)
        }
    }

    fn req<T: FromValue>(&self, pos: usize) -> Result<T, ShellError> {
        if let Some(expression) = self.nth(pos) {
            let value = expression_to_value(expression)?;
            FromValue::from_value(&value)
        } else {
            Err(ShellError::AccessBeyondEnd(
                self.positional.len(),
                self.head,
            ))
        }
    }
}

fn expression_to_value(expression: Expression) -> Result<Value, ShellError> {
    match expression.expr {
        Expr::Bool(val) => Ok(Value::Bool {
            val,
            span: Span::unknown(),
        }),
        Expr::Int(val) => Ok(Value::Int {
            val,
            span: Span::unknown(),
        }),
        Expr::Float(val) => Ok(Value::Float {
            val,
            span: Span::unknown(),
        }),
        Expr::String(val) => Ok(Value::String {
            val,
            span: Span::unknown(),
        }),
        Expr::List(exprs) => {
            let values = exprs
                .into_iter()
                .map(expression_to_value)
                .collect::<Result<Vec<Value>, ShellError>>()?;

            Ok(Value::List {
                vals: values,
                span: Span::unknown(),
            })
        }
        v => Err(ShellError::InternalError(format!(
            "Expression type {:?} not allowed as plugin argument",
            v
        ))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nu_protocol::{
        ast::{Call, Expr, Expression},
        Span, Spanned,
    };

    #[test]
    fn call_to_value() {
        let call = Call {
            decl_id: 1,
            head: Span { start: 0, end: 10 },
            positional: vec![
                Expression {
                    expr: Expr::Float(1.0),
                    span: Span { start: 0, end: 10 },
                    ty: nu_protocol::Type::Float,
                    custom_completion: None,
                },
                Expression {
                    expr: Expr::String("something".into()),
                    span: Span { start: 0, end: 10 },
                    ty: nu_protocol::Type::Float,
                    custom_completion: None,
                },
            ],
            named: vec![
                (
                    Spanned {
                        item: "name".to_string(),
                        span: Span { start: 0, end: 10 },
                    },
                    Some(Expression {
                        expr: Expr::Float(1.0),
                        span: Span { start: 0, end: 10 },
                        ty: nu_protocol::Type::Float,
                        custom_completion: None,
                    }),
                ),
                (
                    Spanned {
                        item: "flag".to_string(),
                        span: Span { start: 0, end: 10 },
                    },
                    None,
                ),
            ],
        };

        let name: Option<f64> = call.get_flag("name").unwrap();
        assert_eq!(name, Some(1.0));

        assert!(call.has_flag("flag"));

        let required: f64 = call.req(0).unwrap();
        assert_eq!(required, 1.0);

        let optional: Option<String> = call.opt(1).unwrap();
        assert_eq!(optional, Some("something".to_string()));

        let rest: Vec<String> = call.rest(1).unwrap();
        assert_eq!(rest, vec!["something".to_string()]);
    }
}
