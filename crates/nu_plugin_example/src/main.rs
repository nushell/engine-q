use nu_plugin::{serve_plugin, CapnpSerializer};
use nu_plugin_example::Example;

fn main() {
    // When defining your plugin, you can select the Serializer that could be
    // used to encode and decode the messages. The available options are
    // CapnpSerializer and JsonSerializer. Both are defined in the serializer
    // folder in nu-plugin.
    serve_plugin(&mut Example {}, CapnpSerializer {})
}
