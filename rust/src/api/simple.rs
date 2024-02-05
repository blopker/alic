use caesium;

pub fn imgcompress(path: String, out_path: String) -> String {
    let pars = caesium::initialize_parameters();
    let result = caesium::compress(
        String::from(path.clone()),
        String::from(out_path.clone()),
        &pars,
    );
    match result {
        Ok(_message) => String::from("Success"),
        Err(err) => "Error: ".to_string() + &err.to_string(),
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
