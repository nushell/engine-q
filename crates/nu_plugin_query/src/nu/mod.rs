use crate::Query;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, Signature, Spanned, SyntaxShape, Value};

impl Plugin for Query {
    fn signature(&self) -> Vec<Signature> {
        vec![
            Signature::build("query")
            .desc("Show all the query commands")
            .category(Category::Experimental),

            Signature::build("query json")
            .desc("execute json query on json file (open --raw <file> | query json 'query string')")
            .required("query", SyntaxShape::String, "json query")
            .category(Category::Experimental),

            Signature::build("query xml")
            .desc("execute xpath query on xml")
            .required("query", SyntaxShape::String, "xpath query")
            .category(Category::Experimental),

            Signature::build("query web")
            .desc("execute selector query on html/web")
            .named("query", SyntaxShape::String, "selector query", Some('q'))
            .switch("as_html", "return the query output as html", Some('m'))
            .named(
                "attribute",
                SyntaxShape::String,
                "downselect based on the given attribute",
                Some('a'),
            )
            .named(
                "as_table",
                SyntaxShape::Table,
                "find table based on column header list",
                Some('t'),
            )
            .switch(
                "inspect",
                "run in inspect mode to provide more information for determining column headers",
                Some('i'),
            )
            .category(Category::Experimental),

            ]
    }

    // fn examples(&self) -> Vec<Example> {
    //     vec![Example {
    //         description: "Some example",
    //         example: "one | two | three",
    //         result: None,
    //     }]
    // }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // You can use the name to identify what plugin signature was called
        let path: Option<Spanned<String>> = call.opt(0)?;

        match name {
            "query" => {
                let help = get_full_help_vec( &Query.signature());
                Ok(Value::string(help, call.head))
                // self.query(name, call, input, path)
            }
            "query json" => self.query_json( name, call, input, path),
            "query web" => self.query_web(name, call, input, path),
            "query xml" => self.query_xml(name, call, input, path),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
        // if name == "query" {
        //     eprintln!("query");
        //     let path: Option<Spanned<String>> = call.opt(0)?;
        //     self.query(input, path, &call.head)
        // } else if name == "query json" {
        //     eprintln!("query json");
        //     let path: Option<Spanned<String>> = call.opt(0)?;
        //     self.query_json(input, path, &call.head)
        // } else {
        //     eprintln!("nothing");
        //     return Ok(Value::Nothing { span: call.head });
        // }
    }
}

pub fn get_full_help_vec(sigs: &[Signature]) -> String {
    let mut help = String::new();
    for x in sigs.iter() {
        help.push_str(&get_documentation(x));
        // eprintln!("{}", get_documentation(x))
    }
    help
}

pub fn get_documentation(sig: &Signature) -> String {
    let cmd_name = &sig.name;
    let mut long_desc = String::new();

    let usage = &sig.usage;
    if !usage.is_empty() {
        long_desc.push_str(usage);
        long_desc.push_str("\n\n");
    }

    // let extra_usage = if config.brief { "" } else { &sig.extra_usage };
    let extra_usage = &sig.extra_usage;
    if !extra_usage.is_empty() {
        long_desc.push_str(extra_usage);
        long_desc.push_str("\n\n");
    }

    // let mut subcommands = vec![];
    // if !config.no_subcommands {
    //     let signatures = engine_state.get_signatures(true);
    //     for sig in signatures {
    //         if sig.name.starts_with(&format!("{} ", cmd_name)) {
    //             subcommands.push(format!("  {} - {}", sig.name, sig.usage));
    //         }
    //     }
    // }

    long_desc.push_str(&format!("Usage:\n  > {}\n", sig.call_signature()));

    // if !subcommands.is_empty() {
    //     long_desc.push_str("\nSubcommands:\n");
    //     subcommands.sort();
    //     long_desc.push_str(&subcommands.join("\n"));
    //     long_desc.push('\n');
    // }

    if !sig.required_positional.is_empty()
        || !sig.optional_positional.is_empty()
        || sig.rest_positional.is_some()
    {
        long_desc.push_str("\nParameters:\n");
        for positional in &sig.required_positional {
            long_desc.push_str(&format!("  {}: {}\n", positional.name, positional.desc));
        }
        for positional in &sig.optional_positional {
            long_desc.push_str(&format!(
                "  (optional) {}: {}\n",
                positional.name, positional.desc
            ));
        }

        if let Some(rest_positional) = &sig.rest_positional {
            long_desc.push_str(&format!("  ...args: {}\n", rest_positional.desc));
        }
    }
    if !sig.named.is_empty() {
        long_desc.push_str(&get_flags_section(sig))
    }

    // if !examples.is_empty() {
    //     long_desc.push_str("\nExamples:");
    // }
    // for example in examples {
    //     long_desc.push('\n');
    //     long_desc.push_str("  ");
    //     long_desc.push_str(example.description);

    //     // if config.no_color {
    //     long_desc.push_str(&format!("\n  > {}\n", example.example));
    //     // }
    //     // else if let Some(highlighter) = engine_state.find_decl(b"nu-highlight") {
    //     //     let decl = engine_state.get_decl(highlighter);

    //     //     if let Ok(output) = decl.run(
    //     //         engine_state,
    //     //         stack,
    //     //         &Call::new(),
    //     //         Value::String {
    //     //             val: example.example.to_string(),
    //     //             span: Span { start: 0, end: 0 },
    //     //         }
    //     //         .into_pipeline_data(),
    //     //     ) {
    //     //         let result = output.into_value(Span { start: 0, end: 0 });
    //     //         match result.as_string() {
    //     //             Ok(s) => {
    //     //                 long_desc.push_str(&format!("\n  > {}\n", s));
    //     //             }
    //     //             _ => {
    //     //                 long_desc.push_str(&format!("\n  > {}\n", example.example));
    //     //             }
    //     //         }
    //     //     }
    //     // }
    // }

    long_desc.push('\n');

    long_desc
}

fn get_flags_section(signature: &Signature) -> String {
    let mut long_desc = String::new();
    long_desc.push_str("\nFlags:\n");
    for flag in &signature.named {
        let msg = if let Some(arg) = &flag.arg {
            if let Some(short) = flag.short {
                if flag.required {
                    format!(
                        "  -{}{} (required parameter) {:?} {}\n",
                        short,
                        if !flag.long.is_empty() {
                            format!(", --{}", flag.long)
                        } else {
                            "".into()
                        },
                        arg,
                        flag.desc
                    )
                } else {
                    format!(
                        "  -{}{} {:?} {}\n",
                        short,
                        if !flag.long.is_empty() {
                            format!(", --{}", flag.long)
                        } else {
                            "".into()
                        },
                        arg,
                        flag.desc
                    )
                }
            } else if flag.required {
                format!(
                    "  --{} (required parameter) {:?} {}\n",
                    flag.long, arg, flag.desc
                )
            } else {
                format!("  --{} {:?} {}\n", flag.long, arg, flag.desc)
            }
        } else if let Some(short) = flag.short {
            if flag.required {
                format!(
                    "  -{}{} (required parameter) {}\n",
                    short,
                    if !flag.long.is_empty() {
                        format!(", --{}", flag.long)
                    } else {
                        "".into()
                    },
                    flag.desc
                )
            } else {
                format!(
                    "  -{}{} {}\n",
                    short,
                    if !flag.long.is_empty() {
                        format!(", --{}", flag.long)
                    } else {
                        "".into()
                    },
                    flag.desc
                )
            }
        } else if flag.required {
            format!("  --{} (required parameter) {}\n", flag.long, flag.desc)
        } else {
            format!("  --{} {}\n", flag.long, flag.desc)
        };
        long_desc.push_str(&msg);
    }
    long_desc
}
