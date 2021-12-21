use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    Any,
    Binary,
    Block,
    Bool,
    CellPath,
    Custom,
    Date,
    Duration,
    Error,
    Expression,
    Filesize,
    Float,
    FullCellPath,
    ImportPattern,
    Int,
    List(Box<Type>),
    MathExpression,
    Nothing,
    Number,
    Operator,
    Range,
    Record(Vec<(String, Type)>),
    Signature,
    String,
    Table,
    Unknown,
    ValueStream,
    Variable,
    VarWithOptType,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Any => write!(f, "any"),
            Type::Binary => write!(f, "binary"),
            Type::Block => write!(f, "block"),
            Type::Bool => write!(f, "bool"),
            Type::CellPath => write!(f, "cell path"),
            Type::Custom => write!(f, "custom"),
            Type::Date => write!(f, "date"),
            Type::Duration => write!(f, "duration"),
            Type::Error => write!(f, "error"),
            Type::Expression => write!(f, "expression"),
            Type::Filesize => write!(f, "filesize"),
            Type::Float => write!(f, "float"),
            Type::FullCellPath => write!(f, "full cellpath"),
            Type::ImportPattern => write!(f, "import pattern"),
            Type::Int => write!(f, "int"),
            Type::List(l) => write!(f, "list<{}>", l),
            Type::MathExpression => write!(f, "math expression"),
            Type::Nothing => write!(f, "nothing"),
            Type::Number => write!(f, "number"),
            Type::Operator => write!(f, "operator"),
            Type::Range => write!(f, "range"),
            Type::Record(fields) => write!(
                f,
                "record<{}>",
                fields
                    .iter()
                    .map(|(x, y)| format!("{}: {}", x, y.to_string()))
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
            Type::Signature => write!(f, "signature"),
            Type::String => write!(f, "string"),
            Type::Table => write!(f, "table"),
            Type::Unknown => write!(f, "unknown"),
            Type::ValueStream => write!(f, "value stream"),
            Type::Variable => write!(f, "variable"),
            Type::VarWithOptType => write!(f, "var with opt type"),
        }
    }
}
