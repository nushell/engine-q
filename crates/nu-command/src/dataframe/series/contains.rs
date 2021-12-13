use super::super::values::{Column, NuDataFrame};

use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape,
};
use polars::prelude::IntoSeries;

#[derive(Clone)]
pub struct Contains;

impl Command for Contains {
    fn name(&self) -> &str {
        "df contains"
    }

    fn usage(&self) -> &str {
        "Checks if a pattern is contained in a string"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required(
                "pattern",
                SyntaxShape::String,
                "Regex pattern to be searched",
            )
            .category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Returns boolean indicating if pattern was found",
            example: "[abc acb acb] | df to-df | df contains ab",
            result: Some(
                NuDataFrame::try_from_columns(vec![Column::new(
                    "0".to_string(),
                    vec![true.into(), false.into(), false.into()],
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
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let df = NuDataFrame::try_from_pipeline(input, call.head)?;
    let pattern: String = call.req(engine_state, stack, 0)?;

    let series = df.as_series(call.head)?;
    let chunked = series.utf8().map_err(|e| {
        ShellError::SpannedLabeledError(
            "The contains command only with string columns".into(),
            e.to_string(),
            call.head,
        )
    })?;

    let res = chunked.contains(&pattern).map_err(|e| {
        ShellError::SpannedLabeledError(
            "Error searching in series".into(),
            e.to_string(),
            call.head,
        )
    })?;

    NuDataFrame::try_from_series(vec![res.into_series()], call.head)
        .map(|df| PipelineData::Value(NuDataFrame::into_value(df, call.head), None))
}

#[cfg(test)]
mod test {
    use super::super::super::test_dataframe::test_dataframe;
    use super::*;

    #[test]
    fn test_examples() {
        test_dataframe(vec![Box::new(Contains {})])
    }
}
