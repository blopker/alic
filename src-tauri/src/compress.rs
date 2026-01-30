use super::settings;
use crate::errors::{AlicError, AlicErrorType};
use crate::events::{AddFileEvent, BadFileEvent};
use crate::macos;
use crate::resize;
use caesium::parameters::CSParameters;
use image::ImageFormat;
use image::{self};
use log::debug;
use specta::Type;
use std::fs::{self};
use std::io::Write;
use std::os::macos::fs::FileTimesExt;
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;
use tauri_specta::Event;

use std::path::{Path, PathBuf};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: String,
    pub file: Option<String>,
    pub status: FileEntryStatus,
    pub size: Option<u32>,
    pub original_size: Option<u32>,
    pub ext: Option<String>,
    pub savings: Option<u32>,
    pub error: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize, Type)]
pub struct FileInfoResult {
    pub size: u32,
    pub extension: String,
    pub filename: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
pub enum FileEntryStatus {
    Processing,
    Compressing,
    Complete,
    AlreadySmaller,
    Error,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize, Type)]
pub enum ImageType {
    JPEG,
    PNG,
    WEBP,
    GIF,
    TIFF,
    AVIF,
}

impl ImageType {
    pub fn extensions(&self) -> &[&str] {
        match self {
            ImageType::JPEG => ImageFormat::Jpeg.extensions_str(),
            ImageType::PNG => ImageFormat::Png.extensions_str(),
            ImageType::WEBP => ImageFormat::WebP.extensions_str(),
            ImageType::GIF => ImageFormat::Gif.extensions_str(),
            ImageType::TIFF => ImageFormat::Tiff.extensions_str(),
            ImageType::AVIF => ImageFormat::Avif.extensions_str(),
        }
    }
    pub fn to_casium_type(&self) -> Option<caesium::SupportedFileTypes> {
        match self {
            ImageType::JPEG => Some(caesium::SupportedFileTypes::Jpeg),
            ImageType::PNG => Some(caesium::SupportedFileTypes::Png),
            ImageType::WEBP => Some(caesium::SupportedFileTypes::WebP),
            ImageType::GIF => Some(caesium::SupportedFileTypes::Gif),
            ImageType::TIFF => Some(caesium::SupportedFileTypes::Tiff),
            ImageType::AVIF => None, // AVIF uses ravif, not libcaesium
        }
    }
    pub fn preferred_extension(&self) -> &str {
        match self {
            ImageType::JPEG => "jpg",
            ImageType::PNG => "png",
            ImageType::WEBP => "webp",
            ImageType::GIF => "gif",
            ImageType::TIFF => "tiff",
            ImageType::AVIF => "avif",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CompressResult {
    pub path: String,
    pub out_size: u32,
    pub out_path: String,
    pub result: String,
}

#[derive(Debug)]
struct ImageData {
    data: Vec<u8>,
    image_type: ImageType,
    size: u64,
    modified: SystemTime,
    created: SystemTime,
}

impl ImageData {
    pub fn new(
        data: Vec<u8>,
        image_type: ImageType,
        size: u64,
        modified: SystemTime,
        created: SystemTime,
    ) -> Self {
        ImageData {
            data,
            image_type,
            size,
            modified,
            created,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn process_img(
    parameters: settings::ProfileData,
    file: FileEntry,
    parallel_images: i32,
) -> Result<CompressResult, AlicError> {
    // check file exists,
    // get type,
    // need conversion?,
    // calculate out path,
    // calculate temp path,
    // compress image to temp path,
    // calculate savings,
    // if not savings, delete temp file, return
    // if out path is same as original, delete original
    // move temp file to out path
    let image_data = match read_image_info(&file.path) {
        Ok(img) => img,
        Err(err) => {
            return Err(AlicError {
                error: err,
                error_type: AlicErrorType::UnsupportedFileType,
            });
        }
    };

    let out_path = get_out_path(&parameters, &file.path, &image_data.image_type);

    if file.path == out_path && !parameters.should_overwrite {
        return Err(AlicError {
            error:
                "Image would be overwritten. Enable \"Allow Overwrite\" in settings to allow this."
                    .to_string(),
            error_type: AlicErrorType::WontOverwrite,
        });
    }

    let should_convert =
        parameters.should_convert && parameters.convert_extension != image_data.image_type;

    // Determine target format for AVIF handling
    let target_format = if should_convert {
        &parameters.convert_extension
    } else {
        &image_data.image_type
    };

    // Handle resize ourselves to get around memory limit issues
    let cs_params = create_cs_parameters(&parameters);

    let data = match parameters.should_resize {
        true => resize::resize(
            image_data.data,
            parameters.resize_width,
            parameters.resize_height,
            parameters.should_background_fill,
            &parameters.background_fill,
            image_data.image_type == ImageType::GIF,
        )?,
        false => image_data.data,
    };

    // AVIF uses ravif directly instead of libcaesium
    let result = if *target_format == ImageType::AVIF {
        compress_avif(&data, &parameters, parallel_images)
    } else if should_convert {
        convert_image(data, cs_params, parameters.convert_extension.clone())
    } else {
        compress_image(data, cs_params, image_data.image_type)
    };

    if result.is_err() {
        return Err(AlicError {
            error: result.err().unwrap().to_string(),
            error_type: AlicErrorType::Unknown,
        });
    }

    let compressed_data = result.unwrap();
    let compressed_size = compressed_data.len() as f64;
    if !parameters.should_convert && compressed_size > image_data.size as f64 * 0.95 {
        return Err(AlicError {
            error: "Image cannot be compressed further.".to_string(),
            error_type: AlicErrorType::NotSmaller,
        });
    }

    if out_path == file.path {
        let res = macos::trash_file(&file.path);
        if res.is_err() {
            return Err(AlicError {
                error: res.err().unwrap().to_string(),
                error_type: AlicErrorType::Unknown,
            });
        }
    }
    let _ = fs::remove_file(&out_path);
    let mut new_file = match fs::File::create_new(&out_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(AlicError {
                error: e.to_string(),
                error_type: AlicErrorType::Unknown,
            });
        }
    };
    let write_result = new_file.write_all(&compressed_data);
    match write_result {
        Ok(_) => {}
        Err(e) => {
            return Err(AlicError {
                error: e.to_string(),
                error_type: AlicErrorType::Unknown,
            });
        }
    };

    if parameters.keep_timestamps {
        let times = fs::FileTimes::new()
            .set_created(image_data.created)
            .set_modified(image_data.modified);
        match new_file.set_times(times) {
            Ok(_) => {}
            Err(e) => {
                return Err(AlicError {
                    error: e.to_string(),
                    error_type: AlicErrorType::Unknown,
                });
            }
        };
    }

    let out_size = compressed_size as u32;
    Ok(CompressResult {
        path: file.path,
        out_size,
        out_path,
        result: "Success".to_string(),
    })
}

fn read_image_info(path: &str) -> Result<ImageData, String> {
    let metadata_result = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(format!("Problem reading file metadata: {err}"));
        }
    };
    let image_bytes = match fs::read(path) {
        Ok(image_bytes) => image_bytes,
        Err(err) => {
            return Err(format!("Problem reading file data: {err}"));
        }
    };
    let format = match image::guess_format(&image_bytes) {
        Ok(format) => format,
        Err(err) => {
            return Err(format!("Unknown format: {err}"));
        }
    };

    let image_type = match format {
        ImageFormat::Jpeg => ImageType::JPEG,
        ImageFormat::Png => ImageType::PNG,
        ImageFormat::WebP => ImageType::WEBP,
        ImageFormat::Gif => ImageType::GIF,
        ImageFormat::Tiff => ImageType::TIFF,
        ImageFormat::Avif => ImageType::AVIF,
        f => {
            let mime_type = f.to_mime_type();
            return Err(format!("Unsupported image type: {mime_type}"));
        }
    };

    Ok(ImageData::new(
        image_bytes,
        image_type,
        metadata_result.size(),
        metadata_result.modified().unwrap_or(SystemTime::now()),
        metadata_result.created().unwrap_or(SystemTime::now()),
    ))
}

fn get_out_path(
    parameters: &settings::ProfileData,
    path: &str,
    guessed_format: &ImageType,
) -> String {
    let path = Path::new(&path);
    let file_extension = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let original_extension = match guessed_format
        .extensions()
        .contains(&file_extension.as_str())
    {
        true => file_extension,
        false => guessed_format.preferred_extension().to_string(),
    };
    let extension = match parameters.should_convert {
        true => parameters
            .convert_extension
            .preferred_extension()
            .to_string(),
        false => original_extension,
    };
    let postfix = match parameters.add_postfix {
        true => parameters.postfix.clone(),
        false => "".to_string(),
    };
    format!("{}{postfix}.{extension}", remove_extension(path))
}

fn create_cs_parameters(parameters: &settings::ProfileData) -> CSParameters {
    let mut cs = CSParameters::new();
    cs.jpeg.quality = parameters.jpeg_quality;
    cs.png.quality = parameters.png_quality;
    cs.webp.quality = parameters.webp_quality;
    cs.gif.quality = parameters.gif_quality;
    cs.png.optimize = !parameters.enable_lossy;
    cs.jpeg.optimize = !parameters.enable_lossy;
    cs.webp.lossless = !parameters.enable_lossy;
    cs.keep_metadata = parameters.keep_metadata;
    cs.jpeg.preserve_icc = true;
    cs
}

// Simplified wrapper functions
fn compress_image(
    original_img_data: Vec<u8>,
    params: CSParameters,
    image_type: ImageType,
) -> Result<Vec<u8>, String> {
    // AVIF should be handled by compress_avif, not here
    if image_type == ImageType::AVIF {
        return Err("AVIF compression should use compress_avif".to_string());
    }
    caesium::compress_in_memory(original_img_data, &params)
        .map_err(|e| format!("Error compressing image: {e}"))
}

fn convert_image(
    original_img_data: Vec<u8>,
    params: CSParameters,
    image_type: ImageType,
) -> Result<Vec<u8>, String> {
    // AVIF should be handled by compress_avif, not here
    match image_type.to_casium_type() {
        Some(caesium_type) => caesium::convert_in_memory(original_img_data, &params, caesium_type)
            .map_err(|e| format!("Error converting image: {e}")),
        None => Err(format!(
            "Cannot convert to {:?} using libcaesium",
            image_type
        )),
    }
}

fn compress_avif(
    original_img_data: &[u8],
    parameters: &settings::ProfileData,
    parallel_images: i32,
) -> Result<Vec<u8>, String> {
    use image::ImageEncoder;
    use image::ImageReader;
    use image::codecs::avif::AvifEncoder;
    use std::io::Cursor;

    // Decode the input image
    let img = ImageReader::new(Cursor::new(original_img_data))
        .with_guessed_format()
        .map_err(|e| format!("Error reading image: {e}"))?
        .decode()
        .map_err(|e| format!("Error decoding image: {e}"))?;

    let rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();

    // Calculate thread count for AVIF encoding
    let cpu_count = num_cpus();
    let avif_threads = if parallel_images <= 1 {
        // Single image, use all available threads
        cpu_count
    } else {
        // Multiple images in parallel, divide threads
        std::cmp::max(1, cpu_count / parallel_images as usize)
    };

    // Quality: 1-100 (1 worst, 100 best)
    let quality = parameters.avif_quality.clamp(1, 100) as u8;
    // Speed: 1-10 (1 slowest/best quality, 10 fastest/worst quality)
    let speed = 4_u8;

    // Create output buffer
    let mut output = Vec::new();
    debug!(
        "Using {avif_threads} AVIF threads with {cpu_count} total threads for {parallel_images} images."
    );

    let encoder = AvifEncoder::new_with_speed_quality(&mut output, speed, quality)
        .with_num_threads(Some(avif_threads));
    encoder
        .write_image(&rgba, width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("Error encoding AVIF: {e}"))?;
    Ok(output)
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
}

fn remove_extension(path: &Path) -> String {
    let result = match path.file_stem() {
        Some(stem) => {
            // Get the parent directory and append the stem to it
            if let Some(parent) = path.parent() {
                parent.join(stem)
            } else {
                PathBuf::from(stem)
            }
        }
        None => path.to_path_buf(),
    };
    result.to_string_lossy().to_string()
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_images(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let on_event = |path: String| AddFileEvent(path).emit(&app).unwrap();

    let file = Path::new(&path);
    if !file.exists() {
        return Ok(());
    }
    if file.is_file() {
        if !is_image(file) {
            BadFileEvent(path).emit(&app).unwrap();
            return Ok(());
        }
        on_event(path);
        return Ok(());
    }
    find_images(path, &on_event);
    Ok(())
}

fn find_images<P: AsRef<Path>, F>(directory: P, on_event: &F)
where
    F: Fn(String),
{
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };

        if path.is_dir() {
            // Recursively search subdirectories
            find_images(&path, on_event);
        } else if is_image(&path) {
            on_event(path.to_string_lossy().to_string());
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_file_info(path: &str) -> Result<FileInfoResult, String> {
    let metadata_result = std::fs::metadata(path);
    let size: u32;
    match metadata_result {
        Ok(metadata) => {
            if let Ok(_size) = metadata.len().try_into() {
                size = _size;
            } else {
                return Err("File too large".to_string());
            }
        }
        Err(err) => {
            return Err(format!("Error getting file size: {err}"));
        }
    }

    let _path = Path::new(&path);

    let extension = _path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let filename = _path.file_name().unwrap().to_string_lossy().to_string();

    Ok(FileInfoResult {
        size,
        extension,
        filename,
    })
}

fn is_image(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    let supported_exts = ["png", "jpeg", "jpg", "gif", "webp", "tiff", "avif"];
    let ext = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_lowercase();
    if !supported_exts.contains(&ext.as_str()) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_image_type() {
        let result = ImageType::JPEG.extensions()[0];
        assert_eq!(result, "jpg".to_string());
    }

    #[test]
    fn test_get_out_path() {
        let mut parameters = settings::ProfileData::new();
        let mut result = get_out_path(&parameters, "test/test.png", &ImageType::PNG);
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        result = get_out_path(&parameters, "test/test.jpeg", &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        result = get_out_path(&parameters, "test/test.jpg", &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpg".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = true;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, "test/test.jpeg", &ImageType::JPEG);
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = false;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, "test/test.jpeg", &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        parameters.add_postfix = false;
        result = get_out_path(&parameters, "test/test.jpeg", &ImageType::PNG);
        assert_eq!(result, "test/test.png".to_string());

        parameters = settings::ProfileData::new();
        parameters.postfix = ".bong".to_string();
        result = get_out_path(&parameters, "test/test.jpeg", &ImageType::PNG);
        assert_eq!(result, "test/test.bong.png".to_string());
    }
}
