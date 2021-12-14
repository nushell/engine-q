mod series;
mod values;

mod append;
mod column;
mod command;
mod describe;
mod drop;
mod dtypes;
mod open;
mod to_df;

pub use series::*;

pub use append::AppendDF;
pub use column::ColumnDF;
pub use command::Dataframe;
pub use describe::DescribeDF;
pub use drop::DropDF;
pub use dtypes::DataTypes;
pub use open::OpenDataFrame;
pub use to_df::ToDataFrame;

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
        ReplaceAll
    );

    // Dataframe commands
    bind_command!(
        AppendDF,
        ColumnDF,
        Dataframe,
        DataTypes,
        DescribeDF,
        DropDF,
        OpenDataFrame,
        ToDataFrame
    );
}

#[cfg(test)]
mod test_dataframe;
