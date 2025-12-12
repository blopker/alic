use std::io::Cursor;

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::imageops::FilterType;
use image::{
    AnimationDecoder, DynamicImage, Frame, GenericImage, GenericImageView, ImageDecoder,
    ImageFormat, ImageReader, Limits,
};
use log::debug;

use crate::errors::AlicError;
use crate::errors::AlicErrorType;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn from_hex(s: &str) -> Result<Color, AlicError> {
        debug!("Parsing hex color: {}", s);
        let hex = s.trim_start_matches('#');

        // Expand 3-character hex to 6-character hex
        let hex = match hex.len() {
            3 => {
                let chars: Vec<char> = hex.chars().collect();
                format!(
                    "{}{}{}{}{}{}",
                    chars[0], chars[0], chars[1], chars[1], chars[2], chars[2]
                )
            }
            6 => hex.to_string(),
            _ => {
                return Err(AlicError {
                    error: format!("Invalid hex color: {s}"),
                    error_type: AlicErrorType::InvalidHexColor,
                });
            }
        };

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| AlicError {
            error: format!("Invalid hex color: {s}"),
            error_type: AlicErrorType::InvalidHexColor,
        })?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| AlicError {
            error: format!("Invalid hex color: {s}"),
            error_type: AlicErrorType::InvalidHexColor,
        })?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| AlicError {
            error: format!("Invalid hex color: {s}"),
            error_type: AlicErrorType::InvalidHexColor,
        })?;
        Ok(Color { r, g, b })
    }
}

pub fn resize(
    image_buffer: Vec<u8>,
    width: u32,
    height: u32,
    should_background_fill: bool,
    background_fill: &str,
    is_gif: bool,
) -> Result<Vec<u8>, AlicError> {
    if is_gif {
        return resize_gif(&image_buffer, width, height);
    }

    let (mut desired_width, mut desired_height) = (width, height);
    let (mut image, format) = read_image(&image_buffer)?;
    if format == ImageFormat::Jpeg {
        let orientation = get_jpeg_orientation(&image_buffer);
        (desired_width, desired_height) = match orientation {
            5..=8 => (height, width),
            _ => (width, height),
        };
    }

    if image.width() >= desired_width || image.height() >= desired_height {
        image = image.resize(desired_width, desired_height, FilterType::Lanczos3);
    }

    if should_background_fill {
        image = add_background(&image, desired_width, desired_height, background_fill)?;
    }

    let mut resized_file_buffer: Vec<u8> = vec![];
    image
        .write_to(&mut Cursor::new(&mut resized_file_buffer), format)
        .map_err(|e| AlicError {
            error: e.to_string(),
            error_type: AlicErrorType::ImageResizeError,
        })?;

    Ok(resized_file_buffer)
}

fn resize_gif(image_buffer: &[u8], width: u32, height: u32) -> Result<Vec<u8>, AlicError> {
    let decoder = GifDecoder::new(Cursor::new(image_buffer)).map_err(|e| AlicError {
        error: e.to_string(),
        error_type: AlicErrorType::ImageResizeError,
    })?;

    let (src_width, src_height) = decoder.dimensions();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|e| AlicError {
            error: e.to_string(),
            error_type: AlicErrorType::ImageResizeError,
        })?;

    // Check if resize is needed
    if src_width <= width && src_height <= height {
        return Ok(image_buffer.to_vec());
    }

    // Calculate new dimensions preserving aspect ratio
    let ratio_w = width as f64 / src_width as f64;
    let ratio_h = height as f64 / src_height as f64;
    let ratio = ratio_w.min(ratio_h);

    let mut new_frames = Vec::new();

    for frame in frames {
        let frame_buffer = frame.buffer();
        let frame_width = frame_buffer.width();
        let frame_height = frame_buffer.height();

        // Calculate new frame dimensions
        let new_frame_width = (frame_width as f64 * ratio).round() as u32;
        let new_frame_height = (frame_height as f64 * ratio).round() as u32;

        // Skip empty frames if any (though unlikely to have 0 dims)
        if new_frame_width == 0 || new_frame_height == 0 {
            continue;
        }

        let dynamic_image = DynamicImage::ImageRgba8(frame_buffer.clone());
        let resized_dynamic =
            dynamic_image.resize(new_frame_width, new_frame_height, FilterType::Lanczos3);
        let resized_buffer = resized_dynamic.into_rgba8();

        let left = (frame.left() as f64 * ratio).round() as u32;
        let top = (frame.top() as f64 * ratio).round() as u32;

        let new_frame = Frame::from_parts(resized_buffer, left, top, frame.delay());
        new_frames.push(new_frame);
    }

    let mut out_buffer = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut out_buffer);
        encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .map_err(|e| AlicError {
                error: e.to_string(),
                error_type: AlicErrorType::ImageResizeError,
            })?;
        encoder
            .encode_frames(new_frames.into_iter())
            .map_err(|e| AlicError {
                error: e.to_string(),
                error_type: AlicErrorType::ImageResizeError,
            })?;
    }

    Ok(out_buffer)
}

fn add_background(
    image: &DynamicImage,
    width: u32,
    height: u32,
    background_fill: &str,
) -> Result<DynamicImage, AlicError> {
    let color = Color::from_hex(background_fill)?;
    let mut bg_image = DynamicImage::new_rgb8(width, height);
    let x_offset = (width - image.width()) / 2;
    let y_offset = (height - image.height()) / 2;
    // Iterate over the coordinates and pixels of the image
    for (x, y, _) in bg_image.clone().pixels() {
        if x >= x_offset
            && x < image.width() + x_offset
            && y >= y_offset
            && y < image.height() + y_offset
        {
            bg_image.put_pixel(x, y, image.get_pixel(x - x_offset, y - y_offset));
        } else {
            bg_image.put_pixel(x, y, image::Rgba([color.r, color.g, color.b, 1]));
        }
    }
    Ok(bg_image)
}

fn read_image(image_buffer: &Vec<u8>) -> Result<(DynamicImage, ImageFormat), AlicError> {
    let mut reader = ImageReader::new(Cursor::new(&image_buffer));
    let mut limits = Limits::default();
    // 2gb limit
    limits.max_alloc = Some(2 * 1024 * 1024 * 1024);
    reader.limits(limits);
    let reader_format = reader.with_guessed_format().map_err(|e| AlicError {
        error: e.to_string(),
        error_type: AlicErrorType::ImageResizeError,
    })?;
    let format = match reader_format.format() {
        Some(format) => format,
        None => {
            return Err(AlicError {
                error: "Unsupported image format".to_string(),
                error_type: AlicErrorType::ImageResizeError,
            });
        }
    };
    let image = reader_format.decode().map_err(|e| AlicError {
        error: e.to_string(),
        error_type: AlicErrorType::ImageResizeError,
    })?;
    Ok((image, format))
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
