use core::time::Duration;
use crossterm::{event::poll, event::Event, event::KeyCode, event::KeyEvent, terminal};
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, Value,
};
use std::io::{stdout, Write};

#[derive(Clone)]
pub struct InputKeys;

impl Command for InputKeys {
    fn name(&self) -> &str {
        "input-keys"
    }

    fn usage(&self) -> &str {
        "Get input from the user."
    }

    fn signature(&self) -> Signature {
        Signature::build("input-keys").category(Category::Platform)
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        // println!("Ready to print events (Abort with ESC):");
        println!("You have 5 seconds to hit a key combination:");
        Ok(print_events()?.into_pipeline_data())
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Get one key event",
            example: "input-keys",
            result: None,
        }]
    }
}

pub fn print_events() -> Result<Value, ShellError> {
    stdout().flush()?;
    terminal::enable_raw_mode()?;
    loop {
        let event = crossterm::event::read()?;
        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }

        print_events_helper(event)?;
    }
    terminal::disable_raw_mode()?;

    Ok(Value::nothing(Span::test_data()))
}

// this fn is totally ripped off from crossterm's examples
// it's really a diagnostic routine to see if crossterm is
// even seeing the events. if you press a key and no events
// are printed, it's a good chance your terminal is eating
// those events.
fn print_events_helper(event: Event) -> Result<Value, ShellError> {
    // loop {
    // Wait up to 5s for another event
    if poll(Duration::from_millis(5_000))? {
        // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
        // let event = crossterm::event::read()?;

        if let Event::Key(KeyEvent { code, modifiers }) = event {
            match code {
                KeyCode::Char(c) => {
                    let record = Value::Record {
                        cols: vec![
                            "char".into(),
                            "code".into(),
                            "modifier".into(),
                            "flags".into(),
                        ],
                        vals: vec![
                            Value::string(format!("{}", c), Span::test_data()),
                            Value::string(format!("{:#08x}", u32::from(c)), Span::test_data()),
                            Value::string(format!("{:?}", modifiers), Span::test_data()),
                            Value::string(format!("{:#08b}", modifiers), Span::test_data()),
                        ],
                        span: Span::test_data(),
                    };
                    return Ok(record);
                }
                _ => {
                    let record = Value::Record {
                        cols: vec!["code".into(), "modifier".into(), "flags".into()],
                        vals: vec![
                            Value::string(format!("{:?}", code), Span::test_data()),
                            Value::string(format!("{:?}", modifiers), Span::test_data()),
                            Value::string(format!("{:#08b}", modifiers), Span::test_data()),
                        ],
                        span: Span::test_data(),
                    };
                    return Ok(record);
                }
            }
        } else {
            let record = Value::Record {
                cols: vec!["event".into()],
                vals: vec![Value::string(format!("{:?}", event), Span::test_data())],
                span: Span::test_data(),
            };
            return Ok(record);
        }

        // hit the esc key to git out
        // if event == Event::Key(KeyCode::Esc.into()) {
        //     break;
        // }
    } else {
        // Timeout expired, no event for 5s
        return Ok(Value::string(
            "Waiting for you to type...".to_string(),
            Span::test_data(),
        ));
    }
    // }

    // Ok(())
    // Ok(Value::nothing(Span::test_data()))
}

#[cfg(test)]
mod tests {
    use super::InputKeys;

    #[test]
    fn examples_work_as_expected() {
        use crate::test_examples;
        test_examples(InputKeys {})
    }
}
