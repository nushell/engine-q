use super::super::values::{Column, NuDataFrame};

use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span,
};
use polars::prelude::IntoSeries;

#[derive(Clone)]
pub struct ArgUnique;

impl Command for ArgUnique {
    fn name(&self) -> &str {
        "df arg-unique"
    }

    fn usage(&self) -> &str {
        "Returns indexes for unique values"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Returns indexes for unique values",
            example: "[1 2 2 3 3] | df to-df | df arg-unique",
            result: Some(
                NuDataFrame::try_from_columns(vec![Column::new(
                    "arg_unique".to_string(),
                    vec![0.into(), 1.into(), 3.into()],
                )])
                .expect("simple df for test should not fail")
                .into_value(Span::unknown()),
            ),
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        command(engine_state, stack, call, input)
    }
}

fn command(
    _engine_state: &EngineState,
    _stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let df = NuDataFrame::try_from_pipeline(input, call.head)?;

    let mut res = df
        .as_series(call.head)?
        .arg_unique()
        .map_err(|e| {
            ShellError::SpannedLabeledError(
                "Error extracting unique values".into(),
                e.to_string(),
                call.head,
            )
        })?
        .into_series();
    res.rename("arg_unique");

    NuDataFrame::try_from_series(vec![res], call.head)
        .map(|df| PipelineData::Value(NuDataFrame::into_value(df, call.head), None))
}

#[cfg(test)]
mod test {
    use super::super::super::test_dataframe::test_dataframe;
    use super::*;

    #[test]
    fn test_examples() {
        test_dataframe(ArgUnique {})
    }
}
