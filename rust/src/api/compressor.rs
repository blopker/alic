use caesium;
use image;
use image::DynamicImage;

pub struct Parameters {
    pub jpeg_quality: u32,
    pub png_quality: u32,
    pub webp_quality: u32,
    pub gif_quality: u32,
    pub resize: bool,
    pub resize_width: u32,
    pub resize_height: u32,
}

pub fn process_img(path: String, out_path: String, parameters: Parameters) -> String {
    let result = read_image(path.clone());

    if result.is_err() {
        return result.unwrap_err();
    }

    let img = result.unwrap();
    let mut height = 0;
    let mut width = 0;

    // set the largest dimension to the resize value, only if the image size is larger than the resize value
    if parameters.resize {
        if img.width() > parameters.resize_width || img.height() > parameters.resize_height {
            if img.width() > img.height() {
                width = parameters.resize_width;
            } else {
                height = parameters.resize_height;
            }
        }
    }

    let result = compress_image(
        path.clone(),
        out_path,
        parameters.jpeg_quality,
        parameters.png_quality,
        parameters.webp_quality,
        parameters.gif_quality,
        width,
        height,
    );
    return result;
}

fn read_image(path: String) -> Result<DynamicImage, String> {
    let image = image::open(path.clone());
    match image {
        Ok(image) => Ok(image),
        Err(err) => Err("Error: ".to_string() + &err.to_string()),
    }
}

fn compress_image(
    path: String,
    out_path: String,
    jpeg_quality: u32,
    png_quality: u32,
    webp_quality: u32,
    gif_quality: u32,
    width: u32,
    height: u32,
) -> String {
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = jpeg_quality;
    pars.png.quality = png_quality;
    pars.webp.quality = webp_quality;
    pars.gif.quality = gif_quality;
    pars.width = width;
    pars.height = height;
    let result = caesium::compress(path, out_path, &mut pars);
    match result {
        Ok(_) => "Success".to_string(),
        Err(err) => "Error: ".to_string() + &err.to_string(),
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
