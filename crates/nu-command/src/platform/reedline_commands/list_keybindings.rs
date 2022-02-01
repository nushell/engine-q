use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, IntoPipelineData, PipelineData, Signature, Span, Value,
};
use reedline::{EditCommand, ReedlineEvent};
use strum::IntoEnumIterator;

#[derive(Clone)]
pub struct ListKeybindings;

impl Command for ListKeybindings {
    fn name(&self) -> &str {
        "keybindings list"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Platform)
    }

    fn usage(&self) -> &str {
        "List available values that can be used to create keybindings"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let columns = vec!["type".to_string(), "name".to_string()];
        let modifiers = vec!["Alt", "Control", "Shift", "None"];

        let all_records = get_modifiers_records(&modifiers, columns.clone(), &call.head)
            .chain(get_event_records(columns.clone(), &call.head))
            .chain(get_edit_records(columns, &call.head))
            .collect::<Vec<Value>>();

        Ok(Value::List {
            vals: all_records,
            span: call.head,
        }
        .into_pipeline_data())
    }
}

/// Return a Vec of the Reedline Keybinding Modifiers
pub fn get_modifiers_records<'a>(
    modifiers: &'a [&'a str],
    columns: Vec<String>,
    span: &'a Span,
) -> impl Iterator<Item = Value> + 'a {
    modifiers.iter().map(move |modifier| {
        let entry_type = Value::String {
            val: "modifier".to_string(),
            span: *span,
        };

        let name = Value::String {
            val: modifier.to_string(),
            span: *span,
        };

        Value::Record {
            cols: columns.clone(),
            vals: vec![entry_type, name],
            span: *span,
        }
    })
}

/// Returns events records
pub fn get_event_records(columns: Vec<String>, span: &Span) -> impl Iterator<Item = Value> + '_ {
    ReedlineEvent::iter().map(move |rle| {
        let entry_type = Value::String {
            val: "event".to_string(),
            span: *span,
        };

        let name = Value::String {
            val: format!("{:?}", rle),
            span: *span,
        };

        Value::Record {
            cols: columns.clone(),
            vals: vec![entry_type, name],
            span: *span,
        }
    })
}

/// Returns edits records
pub fn get_edit_records(columns: Vec<String>, span: &Span) -> impl Iterator<Item = Value> + '_ {
    EditCommand::iter().map(move |edit| {
        let entry_type = Value::String {
            val: "edit".to_string(),
            span: *span,
        };

        let name = Value::String {
            val: format!("{:?}", edit),
            span: *span,
        };

        Value::Record {
            cols: columns.clone(),
            vals: vec![entry_type, name],
            span: *span,
        }
    })
}
