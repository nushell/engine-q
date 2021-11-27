use super::icons::{icon_for_file, iconify_style_ansi};
use lscolors::{LsColors, Style};
use nu_engine::CallExt;
use nu_protocol::{
    ast::{Call, PathMember},
    engine::{Command, EngineState, Stack},
    Category, Config, IntoPipelineData, PipelineData, Signature, Span, SyntaxShape, Value,
};
use nu_term_grid::grid::{Alignment, Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::{Height, Width};

#[derive(Clone)]
pub struct Griddle;

impl Command for Griddle {
    fn name(&self) -> &str {
        "grid"
    }

    fn usage(&self) -> &str {
        "Renders the output to a textual terminal grid."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("grid")
            .named(
                "width",
                SyntaxShape::Int,
                "number of columns wide",
                Some('w'),
            )
            .switch("color", "draw output with color", Some('c'))
            .named(
                "separator",
                SyntaxShape::String,
                "character to separate grid with",
                Some('s'),
            )
            .category(Category::Viewers)
    }

    fn extra_usage(&self) -> &str {
        r#"grid was built to give a concise gridded layout for ls. however,
it determines what to put in the grid by looking for a column named
'name'. this works great for tables and records but for lists we
need to do something different. such as with '[one two three] | grid'
it creates a fake column called 'name' for these values so that it
prints out the list properly."#
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let width_param: Option<String> = call.get_flag(engine_state, stack, "width")?;
        let color_param: bool = call.has_flag("color");
        let separator_param: Option<String> = call.get_flag(engine_state, stack, "separator")?;
        let config = stack.get_config()?;
        let env_str = stack.get_env_var("LS_COLORS");

        match input {
            PipelineData::Value(Value::List { vals, .. }) => {
                // dbg!("value::list");
                let data = convert_to_list2(vals, &config);
                if let Some(items) = data {
                    Ok(create_grid_output(
                        items,
                        call,
                        width_param,
                        color_param,
                        separator_param,
                        env_str,
                    ))
                } else {
                    Ok(PipelineData::new(call.head))
                }
            }
            PipelineData::Stream(stream) => {
                // dbg!("value::stream");
                let data = convert_to_list2(stream, &config);
                if let Some(items) = data {
                    Ok(create_grid_output(
                        items,
                        call,
                        width_param,
                        color_param,
                        separator_param,
                        env_str,
                    ))
                } else {
                    // dbg!(data);
                    Ok(PipelineData::new(call.head))
                }
            }
            PipelineData::Value(Value::Record { cols, vals, .. }) => {
                // dbg!("value::record");
                let mut items = vec![];

                for (i, (c, v)) in cols.into_iter().zip(vals.into_iter()).enumerate() {
                    items.push((i, c, v.into_string(", ", &config)))
                }

                Ok(create_grid_output(
                    items,
                    call,
                    width_param,
                    color_param,
                    separator_param,
                    env_str,
                ))
            }
            x => {
                // dbg!("other value");
                // dbg!(x.get_type());
                Ok(x)
            }
        }
    }
}

pub fn to_nu_ansi_term_color(ls_colors_color: lscolors::Color) -> nu_ansi_term::Color {
    match ls_colors_color {
        lscolors::Color::RGB(r, g, b) => nu_ansi_term::Color::Rgb(r, g, b),
        lscolors::Color::Fixed(n) => nu_ansi_term::Color::Fixed(n),
        lscolors::Color::Black => nu_ansi_term::Color::Black,
        lscolors::Color::Red => nu_ansi_term::Color::Red,
        lscolors::Color::Green => nu_ansi_term::Color::Green,
        lscolors::Color::Yellow => nu_ansi_term::Color::Yellow,
        lscolors::Color::Blue => nu_ansi_term::Color::Blue,
        lscolors::Color::Magenta => nu_ansi_term::Color::Purple,
        lscolors::Color::Cyan => nu_ansi_term::Color::Cyan,
        lscolors::Color::White => nu_ansi_term::Color::White,
    }
}

fn strip_ansi(astring: &str) -> String {
    if let Ok(bytes) = strip_ansi_escapes::strip(astring) {
        String::from_utf8_lossy(&bytes).to_string()
    } else {
        astring.to_string()
    }
}

fn create_grid_output(
    items: Vec<(usize, String, String)>,
    call: &Call,
    width_param: Option<String>,
    color_param: bool,
    separator_param: Option<String>,
    env_str: Option<String>,
) -> PipelineData {
    let ls_colors = match env_str {
        Some(s) => LsColors::from_string(&s),
        None => LsColors::default(),
    };

    let cols = if let Some(col) = width_param {
        col.parse::<u16>().unwrap_or(80)
    } else if let Some((Width(w), Height(_h))) = terminal_size::terminal_size() {
        w
    } else {
        80u16
    };
    let sep = if let Some(separator) = separator_param {
        separator
    } else {
        " │ ".to_string()
    };

    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Text(sep),
    });

    for (_row_index, header, value) in items {
        // only output value if the header name is 'name'
        if header == "name" {
            if color_param {
                let no_ansi = strip_ansi(&value);
                let path = std::path::Path::new(&no_ansi);
                let icon = icon_for_file(path.clone());
                let ls_colors_style = ls_colors.style_for_path(path);
                // eprintln!("ls_colors_style: {:?}", &ls_colors_style);
                let ls_to_ansi = match ls_colors_style {
                    Some(c) => c.to_ansi_term_style(),
                    None => ansi_term::Style::default(),
                };
                // eprintln!("ls_to_ansi: {:?}", &ls_to_ansi);

                let icon_style = iconify_style_ansi(ansi_term::Style {
                    foreground: ls_to_ansi.foreground,
                    background: ls_to_ansi.background,
                    ..Default::default()
                });
                // eprintln!("icon_style: {:?}", &icon_style);

                let ansi_style = ls_colors_style
                    .map(Style::to_crossterm_style)
                    .unwrap_or_default();

                // eprintln!("ansi_style: {:?}", &ansi_style);
                // let xt_icon_style = icon_style
                //     .map(Style::to_crossterm_style)
                //     .unwrap_or_default();
                // let xt_icon_style = Style::to_crossterm_style(icon_style);
                let item = format!(
                    "{} {}",
                    // ansi_style.apply(icon).to_string(),
                    icon_style.paint(icon.to_string()),
                    ansi_style.apply(value).to_string()
                );
                // let mut cell = Cell::from(ansi_style.apply(file_with_icon).to_string());
                let mut cell = Cell::from(item);
                cell.alignment = Alignment::Left;
                grid.add(cell);
            } else {
                let mut cell = Cell::from(value);
                cell.alignment = Alignment::Left;
                grid.add(cell);
            }
        }
    }

    if let Some(grid_display) = grid.fit_into_width(cols as usize) {
        Value::String {
            val: grid_display.to_string(),
            span: call.head,
        }
    } else {
        Value::String {
            val: format!("Couldn't fit grid into {} columns!", cols),
            span: call.head,
        }
    }
    .into_pipeline_data()
}

fn convert_to_list2(
    iter: impl IntoIterator<Item = Value>,
    config: &Config,
) -> Option<Vec<(usize, String, String)>> {
    let mut iter = iter.into_iter().peekable();

    if let Some(first) = iter.peek() {
        let mut headers = first.columns();

        if !headers.is_empty() {
            headers.insert(0, "#".into());
        }

        let mut data = vec![];

        for (row_num, item) in iter.enumerate() {
            let mut row = vec![row_num.to_string()];

            if headers.is_empty() {
                row.push(item.into_string(", ", config))
            } else {
                for header in headers.iter().skip(1) {
                    let result = match item {
                        Value::Record { .. } => {
                            item.clone().follow_cell_path(&[PathMember::String {
                                val: header.into(),
                                span: Span::unknown(),
                            }])
                        }
                        _ => Ok(item.clone()),
                    };

                    match result {
                        Ok(value) => row.push(value.into_string(", ", config)),
                        Err(_) => row.push(String::new()),
                    }
                }
            }

            data.push(row);
        }

        let mut h: Vec<String> = headers.into_iter().collect();

        // This is just a list
        if h.is_empty() {
            // let's fake the header
            h.push("#".to_string());
            h.push("name".to_string());
        }

        // this tuple is (row_index, header_name, value)
        let mut interleaved = vec![];
        for (i, v) in data.into_iter().enumerate() {
            for (n, s) in v.into_iter().enumerate() {
                if h.len() == 1 {
                    // always get the 1th element since this is a simple list
                    // and we hacked the header above because it was empty
                    // 0th element is an index, 1th element is the value
                    interleaved.push((i, h[1].clone(), s))
                } else {
                    interleaved.push((i, h[n].clone(), s))
                }
            }
        }

        Some(interleaved)
    } else {
        None
    }
}
