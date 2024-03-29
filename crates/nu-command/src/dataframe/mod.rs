mod eager;
mod series;
mod values;

pub use eager::add_eager_decls;
pub use series::add_series_decls;

use nu_protocol::engine::StateWorkingSet;

pub fn add_dataframe_decls(working_set: &mut StateWorkingSet) {
    add_series_decls(working_set);
    add_eager_decls(working_set);
}

#[cfg(test)]
mod test_dataframe;
