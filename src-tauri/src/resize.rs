use std::io::Cursor;

use image::ImageReader;
use image::Limits;
use image::imageops::FilterType;

use caesium::error::CaesiumError;

pub fn resize(image_buffer: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, CaesiumError> {
    let (mut desired_width, mut desired_height) = (width, height);
    let mut reader = ImageReader::new(Cursor::new(&image_buffer));
    let mut limits = Limits::default();
    // 2gb limit
    limits.max_alloc = Some(2 * 1024 * 1024 * 1024);
    reader.limits(limits);
    let reader_format = reader.with_guessed_format().map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10300,
    })?;
    let format = match reader_format.format() {
        Some(format) => format,
        None => {
            return Err(CaesiumError {
                message: "Unsupported image format".to_string(),
                code: 1030099,
            });
        }
    };
    if format == image::ImageFormat::Jpeg {
        let orientation = get_jpeg_orientation(&image_buffer);
        (desired_width, desired_height) = match orientation {
            5..=8 => (height, width),
            _ => (width, height),
        };
    }
    let mut image = reader_format.decode().map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 1030199,
    })?;

    let dimensions =
        match compute_dimensions(image.width(), image.height(), desired_width, desired_height) {
            Some(dimensions) => dimensions,
            None => return Ok(image_buffer),
        };
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let mut resized_file: Vec<u8> = vec![];
    image
        .write_to(&mut Cursor::new(&mut resized_file), format)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 1030299,
        })?;

    Ok(resized_file)
}

fn compute_dimensions(
    original_width: u32,
    original_height: u32,
    max_desired_width: u32,
    max_desired_height: u32,
) -> Option<(u32, u32)> {
    // Check if image already fits within max dimensions
    if original_width <= max_desired_width && original_height <= max_desired_height {
        return None;
    }

    // Determine which dimension is the limiting factor by comparing ratios
    // Instead of original_width/max_desired_width vs original_height/max_desired_height,
    // we cross multiply to avoid floating point:
    // original_width * max_desired_height vs original_height * max_desired_width
    let width_limiting = (original_width as u64 * max_desired_height as u64)
        > (original_height as u64 * max_desired_width as u64);

    let (new_width, new_height) = if width_limiting {
        // Width is the limiting factor
        let new_width = max_desired_width;
        let new_height =
            (original_height as u64 * max_desired_width as u64 / original_width as u64) as u32;
        (new_width, new_height)
    } else {
        // Height is the limiting factor
        let new_height = max_desired_height;
        let new_width =
            (original_width as u64 * max_desired_height as u64 / original_height as u64) as u32;
        (new_width, new_height)
    };

    Some((new_width, new_height))
}

fn get_jpeg_orientation(data: &[u8]) -> u32 {
    let reader = exif::Reader::new();
    let mut cursor = Cursor::new(data);

    let exif_data = match reader.read_from_container(&mut cursor) {
        Ok(v) => v,
        Err(_) => return 1,
    };

    let exif_field = match exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
        Some(value) => value,
        None => return 1,
    };

    exif_field.value.get_uint(0).unwrap_or(1)
}
