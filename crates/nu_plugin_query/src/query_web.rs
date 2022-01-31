use crate::web_tables::WebTable;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Spanned, Value};
use scraper::{Html, Selector as ScraperSelector};

pub struct Selector {
    pub query: String,
    pub as_html: bool,
    pub attribute: String,
    pub as_table: Value,
    pub inspect: bool,
}

impl Selector {
    pub fn new() -> Selector {
        Selector {
            query: String::new(),
            as_html: false,
            attribute: String::new(),
            as_table: Value::string("".to_string(), Span::test_data()),
            inspect: false,
        }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_selector_params(
    name: &str,
    call: &EvaluatedCall,
    input: &Value,
    rest: Option<Spanned<String>>,
) -> Result<Value, LabeledError> {
    let query: String = match call.get_flag("query")? {
        Some(q2) => q2,
        None => "".to_string(),
    };
    let as_html = call.has_flag("as_html");
    let attribute: String = match call.get_flag("attribute")? {
        Some(a) => a,
        None => "".to_string(),
    };
    let as_table: Value = match call.get_flag("as_table")? {
        Some(v) => v,
        None => Value::nothing(Span::test_data()),
    };
    let inspect = call.has_flag("inspect");

    let selector = Selector {
        query,
        as_html,
        attribute,
        as_table,
        inspect,
    };

    // if !query.is_empty() && ScraperSelector::parse(&query).is_err() {
    //     return Err(LabeledError {
    //         msg: "Cannot parse this query as a valid css selector".to_string(),
    //         label: "Parse error".to_string(),
    //         span: Some(call.head),
    //     });
    // }

    // match input {
    //     Value::String { val, span } => Ok(begin_selector_query(val.to_string(), selector, *span)),
    //     _ => Err(LabeledError {
    //         label: "requires text input".to_string(),
    //         msg: "Expected text from pipeline".to_string(),
    //         span: Some(input.span()?),
    //     }),
    // }

    // put here to just be able to compile
    Ok(Value::nothing(Span::test_data()))
}

fn begin_selector_query(input_html: String, selector: Selector, span: Span) -> Value {
    // if !selector.as_table.is_empty() {
    //     retrieve_tables(input_html.as_str(), &selector.as_table, selector.inspect)
    // } else {
    //     match selector.attribute.is_empty() {
    //         true => execute_selector_query(
    //             input_html.as_str(),
    //             selector.query.as_str(),
    //             selector.as_html,
    //         ),
    //         false => execute_selector_query_with_attribute(
    //             input_html.as_str(),
    //             selector.query.as_str(),
    //             selector.attribute.as_str(),
    //         ),
    //     }
    // }

    // put here to just be able to compile
    Value::nothing(Span::test_data())
}

pub fn retrieve_tables(input_string: &str, columns: &Value, inspect_mode: bool) -> Value {
    // let html = input_string;
    // // let mut cols = Vec::new();
    // // if let Value::Table(t) = &columns.value {
    // //     for x in t {
    // //         cols.push(x.convert_to_string());
    // //     }
    // // }
    // let cols = columns;

    // if inspect_mode {
    //     eprintln!("Passed in Column Headers = {:#?}", &cols,);
    // }

    // let tables = match WebTable::find_by_headers(html, &cols) {
    //     Some(t) => {
    //         if inspect_mode {
    //             eprintln!("Table Found = {:#?}", &t);
    //         }
    //         t
    //     }
    //     None => vec![WebTable::empty()],
    // };

    // if tables.len() == 1 {
    //     return retrieve_table(
    //         tables
    //             .into_iter()
    //             .next()
    //             .expect("This should never trigger"),
    //         columns,
    //     );
    // }

    // tables
    //     .into_iter()
    //     .map(move |table| Value::Record {
    //         cols: vec![],
    //         vals: retrieve_table(table, columns),
    //         span: Span::test_data(),
    //     })
    //     .collect()

    // put here to just be able to compile
    Value::nothing(Span::test_data())
}

fn retrieve_table(mut table: WebTable, columns: &Value) -> Value {
    // // let mut cols = Vec::new();
    // // if let UntaggedValue::Table(t) = &columns.value {
    // //     for x in t {
    // //         cols.push(x.convert_to_string());
    // //     }
    // // }
    // let cols = columns;

    // // if cols.is_empty() && !table.headers().is_empty() {
    // //     for col in table.headers().keys() {
    // //         cols.push(col.to_string());
    // //     }
    // // }

    // let mut table_out = Vec::new();
    // // sometimes there are tables where the first column is the headers, kind of like
    // // a table has ben rotated ccw 90 degrees, in these cases all columns will be missing
    // // we keep track of this with this variable so we can deal with it later
    // let mut at_least_one_row_filled = false;
    // // if columns are still empty, let's just make a single column table with the data
    // if cols.is_empty() {
    //     at_least_one_row_filled = true;
    //     let table_with_no_empties: Vec<_> = table.iter().filter(|item| !item.is_empty()).collect();

    //     let mut cols = vec![];
    //     let mut vals = vec![];
    //     for row in &table_with_no_empties {
    //         for (counter, cell) in row.iter().enumerate() {
    //             cols.push(format!("Column{}", counter));
    //             vals.push(Value::string(cell.to_string(), Span::test_data()))
    //         }
    //     }
    //     table_out.push(Value::Record {
    //         cols,
    //         vals,
    //         span: Span::test_data(),
    //     })
    // } else {
    //     let mut cols = vec![];
    //     let mut vals = vec![];
    //     for row in &table {
    //         for col in &cols {
    //             let key = col.to_string();
    //             let val = row
    //                 .get(col)
    //                 .unwrap_or(&format!("Missing column: '{}'", &col))
    //                 .to_string();

    //             if !at_least_one_row_filled && val != format!("Missing column: '{}'", &col) {
    //                 at_least_one_row_filled = true;
    //             }
    //             cols.push(key);
    //             vals.push(Value::string(val, Span::test_data()));
    //         }
    //     }
    //     table_out.push(Value::Record {
    //         cols,
    //         vals,
    //         span: Span::test_data(),
    //     })
    // }
    // if !at_least_one_row_filled {
    //     let mut data2 = Vec::new();
    //     for x in &table.data {
    //         data2.push(x.join(", "));
    //     }
    //     table.data = vec![data2];
    //     return retrieve_table(table, columns);
    // }
    // // table_out

    // Value::List {
    //     vals: table.data,
    //     span: Span::test_data(),
    // }

    // put here to just be able to compile
    Value::nothing(Span::test_data())
}

fn execute_selector_query_with_attribute(
    input_string: &str,
    query_string: &str,
    attribute: &str,
) -> Value {
    // let doc = Html::parse_fragment(input_string);

    // doc.select(&css(query_string))
    //     .map(|selection| {
    //         Value::string(
    //             selection.value().attr(attribute).unwrap_or("").to_string(),
    //             Span::test_data(),
    //         )
    //     })
    //     .collect()

    // put here to just be able to compile
    Value::nothing(Span::test_data())
}

fn execute_selector_query(input_string: &str, query_string: &str, as_html: bool) -> Value {
    // let doc = Html::parse_fragment(input_string);

    // match as_html {
    //     true => doc
    //         .select(&css(query_string))
    //         .map(|selection| Value::string(selection.html(), Span::test_data()))
    //         .collect(),
    //     false => doc
    //         .select(&css(query_string))
    //         .map(|selection| {
    //             Value::string(
    //                 selection
    //                     .text()
    //                     .fold("".to_string(), |acc, x| format!("{}{}", acc, x)),
    //                 Span::test_data(),
    //             )
    //         })
    //         .collect(),
    // }

    // put here to just be able to compile
    Value::nothing(Span::test_data())
}

pub fn css(selector: &str) -> ScraperSelector {
    ScraperSelector::parse(selector).expect("this should never trigger")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_LIST: &str = r#"
    <ul>
        <li>Coffee</li>
        <li>Tea</li>
        <li>Milk</li>
    </ul>
"#;

    #[test]
    fn test_first_child_is_not_empty() {
        assert!(!execute_selector_query(SIMPLE_LIST, "li:first-child", false).is_empty())
    }

    // #[test]
    // fn test_first_child() {
    //     assert_eq!(
    //         vec!["Coffee".to_string()],
    //         execute_selector_query(SIMPLE_LIST, "li:first-child", false)
    //     )
    // }
}
