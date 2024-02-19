use caesium;

pub fn imgcompress(
    path: String,
    out_path: String,
    jpeg_quality: u32,
    png_quality: u32,
    webp_quality: u32,
    gif_quality: u32,
) -> String {
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = jpeg_quality;
    pars.png.quality = png_quality;
    pars.webp.quality = webp_quality;
    pars.gif.quality = gif_quality;
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
