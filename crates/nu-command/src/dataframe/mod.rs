mod eager;
mod series;
mod values;

pub use eager::*;
pub use series::*;

use nu_protocol::engine::StateWorkingSet;

pub fn add_dataframe_decls(working_set: &mut StateWorkingSet) {
    macro_rules! bind_command {
            ( $command:expr ) => {
                working_set.add_decl(Box::new($command));
            };
            ( $( $command:expr ),* ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

    // Series commands
    bind_command!(
        AllFalse,
        AllTrue,
        ArgMax,
        ArgMin,
        ArgSort,
        ArgTrue,
        ArgUnique,
        Concatenate,
        Contains,
        Cumulative,
        GetDay,
        GetHour,
        GetMinute,
        GetMonth,
        GetNanosecond,
        GetOrdinal,
        GetSecond,
        GetWeek,
        GetWeekDay,
        GetYear,
        IsDuplicated,
        IsIn,
        IsNotNull,
        IsNull,
        IsUnique,
        NNull,
        NUnique,
        NotSeries,
        Rename,
        Replace,
        ReplaceAll,
        Rolling,
        SetSeries,
        SetWithIndex,
        Shift,
        StrLengths,
        StrSlice,
        StrFTime,
        ToLowerCase,
        ToUpperCase,
        Unique,
        ValueCount
    );

    // Dataframe commands
    bind_command!(
        Aggregate,
        AppendDF,
        ColumnDF,
        CreateGroupBy,
        Dataframe,
        DataTypes,
        DescribeDF,
        DropDF,
        DropNulls,
        OpenDataFrame,
        ToDataFrame,
        WithColumn
    );
}

#[cfg(test)]
mod test_dataframe;
