use nu_protocol::engine::{EngineState, StateWorkingSet};

use crate::*;

pub fn create_default_context() -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // If there are commands that have the same name as default declarations,
        // they have to be registered before the main declarations. This helps to make
        // them only accessible if the correct input value category is used with the
        // declaration
        #[cfg(feature = "dataframe")]
        add_dataframe_decls(&mut working_set);

        bind_command!(
            // === Core === //
            Alias,
            Debug,
            Def,
            Describe,
            Do,
            Echo,
            ExportCommand,
            ExportDef,
            ExportEnv,
            For,
            Help,
            Hide,
            If,
            Let,
            Module,
            Source,
            Use,
            // === Filters === //
            All,
            Any,
            Append,
            Collect,
            Drop,
            Each,
            First,
            Get,
            Last,
            Length,
            Lines,
            Nth,
            ParEach,
            Prepend,
            Range,
            Reject,
            Reverse,
            Select,
            Shuffle,
            Skip,
            SkipUntil,
            SkipWhile,
            Uniq,
            Update,
            Where,
            Wrap,
            Zip,
            // === System === //
            Benchmark,
            External,
            Ps,
            Sys,
            // === Strings === //
            BuildString,
            Format,
            Parse,
            Size,
            Split,
            SplitChars,
            SplitColumn,
            SplitRow,
            Str,
            StrCamelCase,
            StrCapitalize,
            StrCollect,
            StrContains,
            StrDowncase,
            StrEndswith,
            StrFindReplace,
            StrIndexOf,
            StrKebabCase,
            StrLength,
            StrLpad,
            StrPascalCase,
            StrReverse,
            StrRpad,
            StrScreamingSnakeCase,
            StrSnakeCase,
            StrStartsWith,
            StrSubstring,
            StrTrim,
            StrUpcase,
            // === FileSystem === //
            Cd,
            Cp,
            Ls,
            Mkdir,
            Mv,
            Rm,
            Touch,
            // === Platform === //
            Clear,
            Kill,
            Sleep,
            // === Date === //
            Date,
            DateFormat,
            DateHumanize,
            DateListTimezones,
            DateNow,
            DateToTable,
            DateToTimezone,
            // === Shells === //
            Exit,
            // === Formats === //
            From,
            FromCsv,
            FromEml,
            FromIcs,
            FromIni,
            FromJson,
            FromOds,
            FromSsv,
            FromToml,
            FromTsv,
            FromUrl,
            FromVcf,
            FromXlsx,
            FromXml,
            FromYaml,
            FromYml,
            To,
            ToCsv,
            ToHtml,
            ToJson,
            ToMd,
            ToToml,
            ToTsv,
            ToUrl,
            ToXml,
            ToYaml,
            // === Viewers === //
            Griddle,
            Table,
            // === Conversions === //
            Into,
            IntoBinary,
            IntoDatetime,
            IntoDecimal,
            IntoFilesize,
            IntoInt,
            IntoString,
            // === Env === //
            LetEnv,
            WithEnv,
            // === Math === //
            Math,
            MathAbs,
            MathAvg,
            MathCeil,
            MathEval,
            MathFloor,
            MathMax,
            MathMedian,
            MathMin,
            MathMode,
            MathProduct,
            MathRound,
            MathSqrt,
            MathStddev,
            MathSum,
            MathVariance,
            // === Random === //
            Random,
            // === Generators === //
            Cal,
            // === Hash === //
            Hash,
            HashMd5::default(),
            HashSha256::default(),
        );

        #[cfg(feature = "plugin")]
        bind_command!(Register);

        // This is a WIP proof of concept
        // bind_command!(ListGitBranches, Git, GitCheckout, Source);

        working_set.render()
    };

    let _ = engine_state.merge_delta(delta);

    engine_state
}
