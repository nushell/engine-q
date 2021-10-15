use std::{cell::RefCell, rc::Rc};

use nu_engine::eval_block;
use nu_parser::{flatten_block, parse};
use nu_protocol::{
    engine::{EngineState, EvaluationContext, Stack, StateWorkingSet},
    Value,
};
use reedline::Completer;
use serde_json::Value as JsonValue;
use std::process::Command;
use std::str::from_utf8;

const SEP: char = std::path::MAIN_SEPARATOR;

pub struct NuCompleter {
    engine_state: Rc<RefCell<EngineState>>,
}

impl NuCompleter {
    pub fn new(engine_state: Rc<RefCell<EngineState>>) -> Self {
        Self { engine_state }
    }
}

impl Completer for NuCompleter {
    fn complete(&self, line: &str, pos: usize) -> Vec<(reedline::Span, reedline::Suggestion)> {
        let engine_state = self.engine_state.borrow();
        let mut working_set = StateWorkingSet::new(&*engine_state);
        let offset = working_set.next_span_start();
        let pos = offset + pos;
        let (output, _err) = parse(&mut working_set, Some("completer"), line.as_bytes(), false);

        let flattened = flatten_block(&working_set, &output);

        let mut index = 0;
        let mut start = 0;
        let mut end = 0;
        let mut words = vec![];
        let mut current_span = reedline::Span { start: 0, end: 0 };
        for flat in &flattened {
            end = flat.0.end;
            match &flat.1 {
                nu_parser::FlatShape::External | nu_parser::FlatShape::InternalCall => {
                    if flat.0.start > pos {
                        break;
                    }
                    start = index;
                }
                _ => {}
            }
            index += 1;

            let word = working_set.get_span_contents(flat.0);
            let wordstr = String::from_utf8_lossy(word).to_string();

            current_span = reedline::Span {
                start: flat.0.start - offset,
                end: flat.0.end - offset,
            };
            if flat.0.start <= pos && flat.0.end >= pos {
                words.push(wordstr); // TODO crop to pos
                break;
            } else if flat.0.end < pos {
                words.push(wordstr);
            }
            // TODO
        }

        if end < pos {
            current_span = reedline::Span {
                start: pos,
                end: pos,
            };
            words.push("".to_owned());
        }

        let cmd = match words[0].as_str() {
            "example" => "example",
            _ => "carapace",
        };

        let subcmd = match words[0].as_str() {
            "example" => "_carapace",
            _ => &words[0],
        };

        // quick fix multiparts by simply removing `'` and `"`
        let mut cloned = words.clone();
        let patched = words[words.len() - 1].replace("'", "").replace('"', "");
        cloned[words.len() - 1] = patched;

        let output = Command::new(cmd)
            .arg(subcmd)
            .arg("nushell")
            .arg("_")
            .args(cloned)
            .output();

        let output_str = match output {
            Ok(o) => from_utf8(&o.stdout).expect("ignore error").to_owned(),
            _ => "".to_owned(),
        };

        if output_str != "" {
            let empty = serde_json::from_str("[]").expect("ignore error");
            let v: JsonValue = serde_json::from_str(&output_str).unwrap_or(empty);
            let a = v.as_array().expect("ignore error");

            return a
                .into_iter()
                .map(|entry| {
                    let r = entry["replacement"].as_str().expect("ignore error").to_string();
                    let d = entry["display"].as_str().expect("ignore error").to_string();
                    (current_span, reedline::Suggestion{replacement: r, display: d})
                })
                .collect();
        }

        for flat in flattened {
            if pos >= flat.0.start && pos <= flat.0.end {
                let prefix = working_set.get_span_contents(flat.0);
                if prefix.starts_with(b"$") {
                    let mut output = vec![];

                    for scope in &working_set.delta.scope {
                        for v in &scope.vars {
                            if v.0.starts_with(prefix) {
                                output.push((
                                    reedline::Span {
                                        start: flat.0.start - offset,
                                        end: flat.0.end - offset,
                                    },
                                    reedline::Suggestion{
                                        replacement: String::from_utf8_lossy(v.0).to_string(),
                                        display: String::from_utf8_lossy(v.0).to_string(),
                                    },
                                ));
                            }
                        }
                    }
                    for scope in &engine_state.scope {
                        for v in &scope.vars {
                            if v.0.starts_with(prefix) {
                                output.push((
                                    reedline::Span {
                                        start: flat.0.start - offset,
                                        end: flat.0.end - offset,
                                    },
                                    reedline::Suggestion{
                                        replacement: String::from_utf8_lossy(v.0).to_string(),
                                        display: String::from_utf8_lossy(v.0).to_string(),
                                    },
                                ));
                            }
                        }
                    }

                    return output;
                }

                match &flat.1 {
                    nu_parser::FlatShape::Custom(custom_completion) => {
                        let prefix = working_set.get_span_contents(flat.0).to_vec();

                        let (block, ..) =
                            parse(&mut working_set, None, custom_completion.as_bytes(), false);
                        let context = EvaluationContext {
                            engine_state: self.engine_state.clone(),
                            stack: Stack::default(),
                        };
                        let result = eval_block(&context, &block, Value::nothing());

                        let v: Vec<_> = match result {
                            Ok(Value::List { vals, .. }) => vals
                                .into_iter()
                                .map(move |x| {
                                    let s = x.as_string().expect(
                                        "FIXME: better error handling for custom completions",
                                    );

                                    (
                                        reedline::Span {
                                            start: flat.0.start - offset,
                                            end: flat.0.end - offset,
                                        },
                                        reedline::Suggestion{
                                            replacement: s.clone(),
                                            display: s.clone(),
                                        },
                                    )
                                })
                                .filter(|x| x.1.replacement.as_bytes().starts_with(&prefix))
                                .collect(),
                            _ => vec![],
                        };

                        return v;
                    }
                    nu_parser::FlatShape::External | nu_parser::FlatShape::InternalCall => {
                        let prefix = working_set.get_span_contents(flat.0);
                        let results = working_set.find_commands_by_prefix(prefix);

                        return results
                            .into_iter()
                            .map(move |x| {
                                (
                                    reedline::Span {
                                        start: flat.0.start - offset,
                                        end: flat.0.end - offset,
                                    },
                                    reedline::Suggestion{
                                        replacement: String::from_utf8_lossy(&x).to_string(),
                                        display: String::from_utf8_lossy(&x).to_string(),
                                    },
                                )
                            })
                            .collect();
                    }
                    nu_parser::FlatShape::Filepath
                    | nu_parser::FlatShape::GlobPattern
                    | nu_parser::FlatShape::ExternalArg => {
                        let prefix = working_set.get_span_contents(flat.0);
                        let prefix = String::from_utf8_lossy(prefix).to_string();

                        let results = file_path_completion(flat.0, &prefix);

                        return results
                            .into_iter()
                            .map(move |x| {
                                (
                                    reedline::Span {
                                        start: x.0.start - offset,
                                        end: x.0.end - offset,
                                    },
                                    reedline::Suggestion{
                                        replacement: x.1.clone(),
                                        display: x.1.clone(),
                                    },
                                )
                            })
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        vec![]
    }
}

fn file_path_completion(
    span: nu_protocol::Span,
    partial: &str,
) -> Vec<(nu_protocol::Span, String)> {
    use std::path::{is_separator, Path};

    let (base_dir_name, partial) = {
        // If partial is only a word we want to search in the current dir
        let (base, rest) = partial.rsplit_once(is_separator).unwrap_or((".", partial));
        // On windows, this standardizes paths to use \
        let mut base = base.replace(is_separator, &SEP.to_string());

        // rsplit_once removes the separator
        base.push(SEP);
        (base, rest)
    };

    let base_dir = nu_path::expand_path(&base_dir_name);
    // This check is here as base_dir.read_dir() with base_dir == "" will open the current dir
    // which we don't want in this case (if we did, base_dir would already be ".")
    if base_dir == Path::new("") {
        return Vec::new();
    }

    if let Ok(result) = base_dir.read_dir() {
        result
            .filter_map(|entry| {
                entry.ok().and_then(|entry| {
                    let mut file_name = entry.file_name().to_string_lossy().into_owned();
                    if matches(partial, &file_name) {
                        let mut path = format!("{}{}", base_dir_name, file_name);
                        if entry.path().is_dir() {
                            path.push(SEP);
                            file_name.push(SEP);
                        }

                        Some((span, path))
                    } else {
                        None
                    }
                })
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn matches(partial: &str, from: &str) -> bool {
    from.to_ascii_lowercase()
        .starts_with(&partial.to_ascii_lowercase())
}
