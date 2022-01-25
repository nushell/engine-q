mod config_files;
mod eval_file;
mod logger;
mod prompt_update;
mod reedline_config;
mod repl;
mod utils;

#[cfg(test)]
mod tests;

use miette::Result;
use nu_command::create_default_context;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn main() -> Result<()> {
    // miette::set_panic_hook();
    let miette_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |x| {
        crossterm::terminal::disable_raw_mode().expect("unable to disable raw mode");
        miette_hook(x);
    }));

    // Get initial current working directory.
    let init_cwd = utils::get_init_cwd();
    let mut engine_state = create_default_context(&init_cwd);

    // Custom additions
    let delta = {
        let mut working_set = nu_protocol::engine::StateWorkingSet::new(&engine_state);
        working_set.add_decl(Box::new(nu_cli::NuHighlight));

        working_set.render()
    };
    let _ = engine_state.merge_delta(delta, None, &init_cwd);

    // TODO: make this conditional in the future
    // Ctrl-c protection section
    let ctrlc = Arc::new(AtomicBool::new(false));
    let handler_ctrlc = ctrlc.clone();
    let engine_state_ctrlc = ctrlc.clone();

    ctrlc::set_handler(move || {
        handler_ctrlc.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    engine_state.ctrlc = Some(engine_state_ctrlc);
    // End ctrl-c protection section

    let mut args_to_nushell = vec![];
    let mut script_name = String::new();
    let mut args_to_script = vec![];

    // Would be nice if we had a way to parse this. The first flags we see will be going to nushell
    // then it'll be the script name
    // then the args to the script

    let mut collect_arg_nushell = false;
    for arg in std::env::args().skip(1) {
        if !script_name.is_empty() {
            args_to_script.push(arg);
        } else if collect_arg_nushell {
            args_to_nushell.push(arg);
            collect_arg_nushell = false;
        } else if arg.starts_with('-') {
            // Cool, it's a flag
            if arg == "-c"
                || arg == "--commands"
                || arg == "--develop"
                || arg == "--debug"
                || arg == "--loglevel"
                || arg == "--config-file"
            {
                collect_arg_nushell = true;
            }
            args_to_nushell.push(arg);
        } else {
            // Our script file
            script_name = arg;
        }
    }

    if let Some(path) = std::env::args().nth(1) {
        eval_file::evaluate(path, &args_to_script, init_cwd, &mut engine_state)
    } else {
        repl::evaluate(ctrlc, &mut engine_state)
    }
}
