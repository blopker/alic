use caesium;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub fn imgcompress(path: String, out_path: String) -> String {
    let pars = caesium::initialize_parameters();
    let result = caesium::compress(
        String::from(path.clone()),
        String::from(out_path.clone()),
        &pars,
    );
    match result {
        Ok(_message) => String::from("Success"),
        Err(err) => err.to_string(),
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
