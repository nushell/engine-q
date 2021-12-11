use nu_plugin::{serve_plugin, JsonSerializer};
use nu_plugin_example::Example;

fn main() {
    eprintln!("before the call");
    serve_plugin(&mut Example {}, JsonSerializer {})
}
