use indexmap::IndexMap;
use nu_engine::CallExt;
use nu_protocol::ast::{Call, CellPath, PathMember};

use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example,  PipelineData, ShellError,
    Signature, Span, SyntaxShape,  Value,
};

#[derive(Clone)]
pub struct Flatten;

impl Command for Flatten {
    fn name(&self) -> &str {
        "flatten"
    }

    fn signature(&self) -> Signature {
        Signature::build("flatten")
            .rest(
                "rest",
                SyntaxShape::String,
                "optionally flatten data by column",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Flatten the table."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        flatten(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "flatten a table",
                example: "[[N, u, s, h, e, l, l]] | flatten ",
                result: None
            
            },
            Example {
                description: "flatten a table",
                example: "[[N, u, s, h, e, l, l]] | flatten | first",
                result: None,
            },
            Example {
                description: "flatten a column having a nested table",
                example: "[[origin, people]; [Ecuador, ([[name, meal]; ['Andres', 'arepa']])]] | flatten | get meal",
                result: None,
            },
            Example {
                description: "restrict the flattening by passing column names",
                example: "[[origin, crate, versions]; [World, ([[name]; ['nu-cli']]), ['0.21', '0.22']]] | flatten versions | last | get versions",
                result: None,
            }
        ]
    }
}

fn flatten(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let tag = call.head;
    let columns: Vec<CellPath> = call.rest(engine_state, stack, 0)?;
    
    input.flat_map(
        move |item| flat_value(&columns, &item, tag),
        engine_state.ctrlc.clone(),
    )
}

enum TableInside<'a> {
    Entries(&'a str, &'a Span, Vec<&'a Value>),
}

fn is_table(value: &Value) -> bool {
 match value {
        Value::List { vals, span: _ } => vals.iter().all(|f| f.as_record().is_ok()),
        _ => false,
    }
}

fn flat_value(columns: &[CellPath], item: &Value, _name_tag: Span) -> Vec<Value> {
    let tag = item.span().unwrap();

    let res = {
        if item.as_record().is_ok() {
            let mut out = TaggedDictBuilder::new(tag);
            let mut out1 = IndexMap::<String, Value>::new();
            let mut a_table = None;
            let mut tables_explicitly_flattened = 0;

            let x = match item {
                Value::Record { cols, vals, span: _ } => (cols, vals),
                _ => todo!(),
            };

            let s = item.span().unwrap();

            for (column, value) in x.0.iter().zip(x.1.iter()) {
                let column_requested = columns.iter().find(|c| c.into_string() == *column);

                match value {
                    Value::List { vals, span: _ }
                        if vals.iter().all(|f| f.as_record().is_ok()) =>
                    {
                        let mut cs = vec![];
                        let mut vs = vec![];

                        for v in vals {
                            if let Ok(r) = v.as_record() {
                                cs.push(r.0);
                                vs.push(r.1)
                            }
                        }

                        if column_requested.is_none() && !columns.is_empty() {
                            
                            if out.contains_key(column) {
                                out.insert_value(format!("{}_{}", column, column), value.clone());
                            } else {
                                out.insert_value(column, value.clone());
                            }
                            continue;
                        }
                        
                        let cols = cs.into_iter().flat_map(|f| f.to_vec());
                        let vals = vs.into_iter().flat_map(|f| f.to_vec());

                        for (k, v) in cols.into_iter().zip(vals.into_iter()) {
                            if out.contains_key(&k) {
                                out.insert_value(format!("{}_{}", column, k), v.clone());
                            } else {
                                out.insert_value(k, v.clone());
                            }
                        }
                    }
                    Value::List { vals: _, span: _ } => {
                     
                        let vals = if let Value::List { vals, span: _ } = value {
                            vals.iter().collect::<Vec<_>>()
                        } else {
                            vec![]
                        };
    
                        if tables_explicitly_flattened >= 1 && column_requested.is_some() {
                          
    
                            return vec![Value::Error{ error: ShellError::UnsupportedInput(
                                    "can only flatten one inner table at the same time. tried flattening more than one column with inner tables... but is flattened already".to_string(),
                                    s
                                )}
    
                            ];
                        }
    
                        if !columns.is_empty() {
                            let cell_path = match column_requested {
                                Some(x) => match x.members.first() {
                                    Some(PathMember::String { val, span:_ }) => Some(val),
                                    Some(PathMember::Int { val:_, span:_ }) => None,
                                    None => None,
                                },
                                None => None,
                            };

                            if let Some(r) = cell_path {
                                if !columns.is_empty() {
                            
                            
                              
                                    a_table = Some(TableInside::Entries(
                                        r,
                                        &s,
                                        vals.into_iter().collect::<Vec<_>>(),
                                    ));
    
                                    tables_explicitly_flattened += 1;
                                }
                            
                            } else {
                                out.insert_value(column, value.clone());
                            }
                        } else if a_table.is_none() {
                            a_table = Some(TableInside::Entries(
                                column,
                                &s,
                                vals.into_iter().collect::<Vec<_>>(),
                            ))
                        }
                    },
                        
                    _ => {out.insert_value(column, value.clone())}
                    
                }
            }
            

            let mut expanded = vec![];
            
            if let Some(TableInside::Entries(column, _, entries)) = a_table {
                for entry in entries {
                    let mut base = out.clone();
                    base.insert_value(column, entry.clone());
                    expanded.push(base.into_value());
                }
            } else {
                expanded.push(out.into_value());
            }

            expanded
            // vec![]
        
        } else if !is_table(item) {
            if let Value::List { vals, span:_ } = item {
                vals.to_vec()
            } else {
                vec![]
            }
        } else {
            vec![item.clone()]
        }
    };
    res
}

/// A helper to help create dictionaries for you. It has the ability to insert values into the dictionary while maintaining the tags that need to be applied to the individual members
#[derive(Debug, Clone)]
pub struct TaggedDictBuilder {
    tag: Span,
    dict: IndexMap<String, Value>,
}

impl TaggedDictBuilder {
    /// Create a new builder
    pub fn new(tag: impl Into<Span>) -> TaggedDictBuilder {
        TaggedDictBuilder {
            tag: tag.into(),
            dict: IndexMap::default(),
        }
    }

    /// Build the contents of the builder into a Value
    pub fn build(tag: impl Into<Span>, block: impl FnOnce(&mut TaggedDictBuilder)) -> Value {
        let mut builder = TaggedDictBuilder::new(tag);
        block(&mut builder);
        builder.into_value()
    }

    /// Create a new builder with a pre-defined capacity
    pub fn with_capacity(tag: impl Into<Span>, n: usize) -> TaggedDictBuilder {
        TaggedDictBuilder {
            tag: tag.into(),
            dict: IndexMap::with_capacity(n),
        }
    }

    /// Insert an untagged key/value pair into the dictionary, to later be tagged when built
    pub fn insert_untagged(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.dict.insert(key.into(), value.into());
    }

    ///  Insert a key/value pair into the dictionary
    pub fn insert_value(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.dict.insert(key.into(), value.into());
    }

    /// Convert the dictionary into a tagged Value using the original tag
    pub fn into_value(self) -> Value {
        let tag = self.tag;
        let cols = self.dict.keys().map(|f| f.to_string()).collect::<Vec<_>>();
        let vals =  self.dict.values().cloned().collect(); //self.dict.values().map(|f| f.clone()).collect::<Vec<_>>();
        Value::Record {
            cols, //: self.dict.keys().collect::<Vec<_>>(),
            vals, //: self.dict.values().collect::<Vec<_>>(),
            span: tag,
        }
    }

    /// Returns true if the dictionary is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.dict.is_empty()
    }

    /// Checks if given key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.dict.contains_key(key)
    }
}

impl From<TaggedDictBuilder> for Value {
    /// Convert a builder into a tagged Value
    fn from(input: TaggedDictBuilder) -> Value {
        input.into_value()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Flatten {})
    }
}
