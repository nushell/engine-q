use crate::formats::to::delimited::merge_descriptors;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Config, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, Spanned, SyntaxShape,
    Value,
};

#[derive(Clone)]
pub struct ToMd;

impl Command for ToMd {
    fn name(&self) -> &str {
        "to md"
    }

    fn signature(&self) -> Signature {
        Signature::build("to md")
            .switch(
                "pretty",
                "Formats the Markdown table to vertically align items",
                Some('p'),
            )
            .switch(
                "per-element",
                "treat each row as markdown syntax element",
                Some('e'),
            )
    }

    fn usage(&self) -> &str {
        "Convert table into simple Markdown"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let head = call.head;
        let pretty = call.has_flag("pretty");
        let per_element = call.has_flag("per-element");
        let config = stack.get_config()?;
        to_md(input, pretty, per_element, config, head)
    }

}

fn to_md(input: PipelineData, pretty: bool, per_element: bool, config: Config, head: Span) -> Result<PipelineData, ShellError> {
    Ok(Value::test_string(
        match input.into_value(head) {
            Value::List { vals, span } => table(&vals, pretty, &config),
            _ => "".to_string()
        }
    ).into_pipeline_data())
}

fn fragment(input: Value, pretty: bool, config: &Config) -> String {
    let headers = match input {
        Value::Record { ref cols, .. } => cols.to_owned(),
        _ => vec!["".to_string()],
    };
    let mut out = String::new();

    if headers.len() == 1 {
        let markup = match (&headers[0]).to_ascii_lowercase().as_ref() {
            "h1" => "# ".to_string(),
            "h2" => "## ".to_string(),
            "h3" => "### ".to_string(),
            "blockquote" => "> ".to_string(),

            _ => return table(&[input.clone()], pretty, config),
        };

        out.push_str(&markup);
        out.push_str(&input.into_string("|", config));
    // } else if let Value::Record { cols, vals, span } = input {
    //     let string = match vals.iter().next() {
    //         Some(value) => value.1.as_string().unwrap_or_default(),
    //         None => String::from(""),
    //     };

    //     out = format_leaf(&UntaggedValue::from(string)).plain_string(100_000)
    } else {
        out = input.into_string("|", config)
    }

    out.push('\n');
    out
}

fn collect_headers(headers: &[String]) -> (Vec<String>, Vec<usize>) {
    let mut escaped_headers: Vec<String> = Vec::new();
    let mut column_widths: Vec<usize> = Vec::new();

    if !headers.is_empty() && (headers.len() > 1 || !headers[0].is_empty()) {
        for header in headers {
            let escaped_header_string = htmlescape::encode_minimal(header);
            column_widths.push(escaped_header_string.len());
            escaped_headers.push(escaped_header_string);
        }
    } else {
        column_widths = vec![0; headers.len()]
    }

    (escaped_headers, column_widths)
}

fn table(input: &[Value], pretty: bool, config: &Config) -> String {
    let headers = merge_descriptors(input);

    let (escaped_headers, mut column_widths) = collect_headers(&headers);

    let mut escaped_rows: Vec<Vec<String>> = Vec::new();

    for row in input {
        let mut escaped_row: Vec<String> = Vec::new();

        match row.to_owned() {
            Value::Record { cols, vals, span } => {
                for i in 0..headers.len() {
                    let data = row.get_data_by_key(&headers[i]);
                    let value_string = data.unwrap_or_else(|| Value::nothing(span)).into_string("|", config);
                    let new_column_width = value_string.len();

                    escaped_row.push(value_string);

                    if column_widths[i] < new_column_width {
                        column_widths[i] = new_column_width;
                    }
                }
            }
            p => {
                let value_string =
                    htmlescape::encode_minimal(&p.into_string("|", config));
                escaped_row.push(value_string);
            }
        }

        escaped_rows.push(escaped_row);
    }

    let output_string = if (column_widths.is_empty() || column_widths.iter().all(|x| *x == 0))
        && escaped_rows.is_empty()
    {
        String::from("")
    } else {
        get_output_string(&escaped_headers, &escaped_rows, &column_widths, pretty)
            .trim()
            .to_string()
    };

    output_string
}

fn get_output_string(
    headers: &[String],
    rows: &[Vec<String>],
    column_widths: &[usize],
    pretty: bool,
) -> String {
    let mut output_string = String::new();

    if !headers.is_empty() {
        output_string.push('|');

        for i in 0..headers.len() {
            if pretty {
                output_string.push(' ');
                output_string.push_str(&get_padded_string(
                    headers[i].clone(),
                    column_widths[i],
                    ' ',
                ));
                output_string.push(' ');
            } else {
                output_string.push_str(&headers[i]);
            }

            output_string.push('|');
        }

        output_string.push_str("\n|");

        #[allow(clippy::needless_range_loop)]
        for i in 0..headers.len() {
            if pretty {
                output_string.push(' ');
                output_string.push_str(&get_padded_string(
                    String::from("-"),
                    column_widths[i],
                    '-',
                ));
                output_string.push(' ');
            } else {
                output_string.push('-');
            }

            output_string.push('|');
        }

        output_string.push('\n');
    }

    for row in rows {
        if !headers.is_empty() {
            output_string.push('|');
        }

        for i in 0..row.len() {
            if pretty {
                output_string.push(' ');
                output_string.push_str(&get_padded_string(row[i].clone(), column_widths[i], ' '));
                output_string.push(' ');
            } else {
                output_string.push_str(&row[i]);
            }

            if !headers.is_empty() {
                output_string.push('|');
            }
        }

        output_string.push('\n');
    }

    output_string
}

fn get_padded_string(text: String, desired_length: usize, padding_character: char) -> String {
    let repeat_length = if text.len() > desired_length {
        0
    } else {
        desired_length - text.len()
    };

    format!(
        "{}{}",
        text,
        padding_character.to_string().repeat(repeat_length)
    )
}

fn one(string: &str) -> String {
    string
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n")
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{fragment, one, table};
    use nu_protocol::{row, Value};

    #[test]
    fn render_h1() {
        let value = row! {"H1".into() => Value::from("Ecuador")};

        assert_eq!(fragment(&value, false), "# Ecuador\n");
    }

    #[test]
    fn render_h2() {
        let value = row! {"H2".into() => Value::from("Ecuador")};

        assert_eq!(fragment(&value, false), "## Ecuador\n");
    }

    #[test]
    fn render_h3() {
        let value = row! {"H3".into() => Value::from("Ecuador")};

        assert_eq!(fragment(&value, false), "### Ecuador\n");
    }

    #[test]
    fn render_blockquote() {
        let value = row! {"BLOCKQUOTE".into() => Value::from("Ecuador")};

        assert_eq!(fragment(&value, false), "> Ecuador\n");
    }

    #[test]
    fn render_table() {
        let value = vec![
            row! { "country".into() => Value::from("Ecuador")},
            row! { "country".into() => Value::from("New Zealand")},
            row! { "country".into() => Value::from("USA")},
        ];

        assert_eq!(
            table(&value, false),
            one(r#"
            |country|
            |-|
            |Ecuador|
            |New Zealand|
            |USA|
        "#)
        );

        assert_eq!(
            table(&value, true),
            one(r#"
            | country     |
            | ----------- |
            | Ecuador     |
            | New Zealand |
            | USA         |
        "#)
        );
    }
}
