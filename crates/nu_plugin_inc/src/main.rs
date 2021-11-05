use nu_plugin::serve_plugin;
use nu_plugin_inc::Inc;

use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    trace!("main main");
    serve_plugin(&mut Inc::new())
}
