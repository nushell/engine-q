mod nu_dataframe;

use nu_dataframe::*;
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

    bind_command!(
        AppendDF,
        ColumnDF,
        DataTypes,
        DescribeDF,
        DropDF,
        OpenDataFrame,
        ToDataFrame
    );
}
