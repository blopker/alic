use crate::events::{AddFileEvent, BadFileEvent};
use crate::macos;

use super::settings;
use caesium::parameters::CSParameters;
use image::ImageFormat;
use image::{self};
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

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize, Type)]
pub enum ImageType {
    JPEG,
    PNG,
    WEBP,
    GIF,
    TIFF,
}

impl ImageType {
    // pub fn to_mime_type(&self) -> String {
    //     match self {
    //         ImageType::JPEG => "image/jpeg".to_string(),
    //         ImageType::PNG => "image/png".to_string(),
    //         ImageType::WEBP => "image/webp".to_string(),
    //         ImageType::GIF => "image/gif".to_string(),
    //         ImageType::TIFF => "image/tiff".to_string(),
    //     }
    // }
    pub fn extensions(&self) -> &[&str] {
        match self {
            ImageType::JPEG => ImageFormat::Jpeg.extensions_str(),
            ImageType::PNG => ImageFormat::Png.extensions_str(),
            ImageType::WEBP => ImageFormat::WebP.extensions_str(),
            ImageType::GIF => ImageFormat::Gif.extensions_str(),
            ImageType::TIFF => ImageFormat::Tiff.extensions_str(),
        }
    }
    // pub fn from_extension(ext: &str) -> Option<Self> {
    //     match ext {
    //         "jpg" | "jpeg" => Some(ImageType::JPEG),
    //         "png" => Some(ImageType::PNG),
    //         "webp" => Some(ImageType::WEBP),
    //         "gif" => Some(ImageType::GIF),
    //         "tiff" => Some(ImageType::TIFF),
    //         _ => None,
    //     }
    // }
    pub fn to_casium_type(&self) -> caesium::SupportedFileTypes {
        match self {
            ImageType::JPEG => caesium::SupportedFileTypes::Jpeg,
            ImageType::PNG => caesium::SupportedFileTypes::Png,
            ImageType::WEBP => caesium::SupportedFileTypes::WebP,
            ImageType::GIF => caesium::SupportedFileTypes::Gif,
            ImageType::TIFF => caesium::SupportedFileTypes::Tiff,
        }
    }
    pub fn preferred_extension(&self) -> &str {
        match self {
            ImageType::JPEG => "jpg",
            ImageType::PNG => "png",
            ImageType::WEBP => "webp",
            ImageType::GIF => "gif",
            ImageType::TIFF => "tiff",
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

#[derive(serde::Serialize, serde::Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CompressError {
    pub error: String,
    pub error_type: CompressErrorType,
}

#[derive(serde::Serialize, serde::Deserialize, Type)]
pub enum CompressErrorType {
    Unknown,
    FileTooLarge,
    FileNotFound,
    UnsupportedFileType,
    WontOverwrite,
    NotSmaller,
}

#[derive(Debug)]
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<u8>,
    image_type: ImageType,
    size: u64,
    modified: SystemTime,
    created: SystemTime,
}

impl ImageData {
    pub fn new(
        width: u32,
        height: u32,
        data: Vec<u8>,
        image_type: ImageType,
        size: u64,
        modified: SystemTime,
        created: SystemTime,
    ) -> Self {
        ImageData {
            width,
            height,
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
) -> Result<CompressResult, CompressError> {
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
            return Err(CompressError {
                error: err,
                error_type: CompressErrorType::UnsupportedFileType,
            });
        }
    };

    let out_path = get_out_path(&parameters, &file.path, &image_data.image_type);

    if file.path == out_path && !parameters.should_overwrite {
        return Err(CompressError {
            error:
                "Image would be overwritten. Enable \"Allow Overwrite\" in settings to allow this."
                    .to_string(),
            error_type: CompressErrorType::WontOverwrite,
        });
    }

    let cs_params = create_cs_parameters(&parameters, image_data.width, image_data.height);

    let should_convert =
        parameters.should_convert && parameters.convert_extension != image_data.image_type;

    let result = match image_data.image_type {
        ImageType::GIF => {
            if should_convert {
                convert_image_from_file(
                    Path::new(&file.path),
                    cs_params,
                    parameters.convert_extension,
                )
            } else {
                compress_image_from_file(Path::new(&file.path), cs_params)
            }
        }
        _ => {
            if should_convert {
                convert_image(image_data.data, cs_params, parameters.convert_extension)
            } else {
                compress_image(image_data.data, cs_params)
            }
        }
    };

    if result.is_err() {
        return Err(CompressError {
            error: result.err().unwrap().to_string(),
            error_type: CompressErrorType::Unknown,
        });
    }

    let compressed_data = result.unwrap();
    let compressed_size = compressed_data.len() as f64;
    if !parameters.should_convert && compressed_size > image_data.size as f64 * 0.95 {
        return Err(CompressError {
            error: "Image cannot be compressed further.".to_string(),
            error_type: CompressErrorType::NotSmaller,
        });
    }

    if out_path == file.path {
        let res = macos::trash_file(&file.path);
        if res.is_err() {
            return Err(CompressError {
                error: res.err().unwrap().to_string(),
                error_type: CompressErrorType::Unknown,
            });
        }
    }
    let mut new_file = fs::File::create_new(&out_path).unwrap();
    let write_result = new_file.write_all(&compressed_data);
    match write_result {
        Ok(_) => {}
        Err(e) => {
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::Unknown,
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
                return Err(CompressError {
                    error: e.to_string(),
                    error_type: CompressErrorType::Unknown,
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

fn get_temp_path(path: &Path) -> String {
    // /original/path/test.png -> /original/path/.test.png
    let path = Path::new(&path);
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    let directory = path.parent().unwrap().to_string_lossy().to_string();
    Path::new(&directory)
        .join(format!(".{}", filename))
        .to_string_lossy()
        .to_string()
}

fn read_image_info(path: &str) -> Result<ImageData, String> {
    let metadata_result = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(format!("Problem reading file metadata: {}", err));
        }
    };
    let image_bytes = match fs::read(path) {
        Ok(image_bytes) => image_bytes,
        Err(err) => {
            return Err(format!("Problem reading file data: {}", err));
        }
    };
    let format = match image::guess_format(&image_bytes) {
        Ok(format) => format,
        Err(err) => {
            return Err(format!("Unknown format: {}", err));
        }
    };
    let image = match image::load_from_memory_with_format(&image_bytes, format) {
        Ok(image) => image,
        Err(err) => {
            return Err(format!("Issue decoding file: {}", err));
        }
    };

    let image_type = match format {
        ImageFormat::Jpeg => ImageType::JPEG,
        ImageFormat::Png => ImageType::PNG,
        ImageFormat::WebP => ImageType::WEBP,
        ImageFormat::Gif => ImageType::GIF,
        ImageFormat::Tiff => ImageType::TIFF,
        f => {
            return Err(format!("Unsupported image type: {}", f.to_mime_type()));
        }
    };

    Ok(ImageData::new(
        image.width(),
        image.height(),
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
    format!("{}{}.{}", remove_extension(path), postfix, extension)
}

fn create_cs_parameters(
    parameters: &settings::ProfileData,
    width: u32,
    height: u32,
) -> CSParameters {
    let mut new_height = 0;
    let mut new_width = 0;

    // set the largest dimension to the resize value,
    // only if the image size is larger than the resize value
    if parameters.should_resize
        && (width > parameters.resize_width || height > parameters.resize_height)
    {
        if width > height {
            new_width = parameters.resize_width;
        } else {
            new_height = parameters.resize_height;
        }
    }

    let mut cs = CSParameters::new();
    cs.jpeg.quality = parameters.jpeg_quality;
    cs.png.quality = parameters.png_quality;
    cs.webp.quality = parameters.webp_quality;
    cs.gif.quality = parameters.gif_quality;
    cs.width = new_width;
    cs.height = new_height;
    cs.optimize = !parameters.enable_lossy;
    cs.keep_metadata = parameters.keep_metadata;
    cs
}

enum ImageOperation {
    Compress,
    Convert(ImageType),
}

enum ImageSource {
    Memory(Vec<u8>),
    File(PathBuf),
}

fn process_image(
    original_img: ImageSource,
    params: CSParameters,
    operation: ImageOperation,
) -> Result<Vec<u8>, String> {
    match original_img {
        ImageSource::Memory(data) => match operation {
            ImageOperation::Compress => caesium::compress_in_memory(data, &params)
                .map_err(|e| format!("Error compressing image: {}", e)),
            ImageOperation::Convert(image_type) => {
                caesium::convert_in_memory(data, &params, image_type.to_casium_type())
                    .map_err(|e| format!("Error converting image: {}", e))
            }
        },
        ImageSource::File(path) => {
            let temp_path = get_temp_path(&path);

            // Perform the operation
            let result = match operation {
                ImageOperation::Compress => caesium::compress(
                    path.to_string_lossy().to_string(),
                    temp_path.clone(),
                    &params,
                ),
                ImageOperation::Convert(image_type) => caesium::convert(
                    path.to_string_lossy().to_string(),
                    temp_path.clone(),
                    &params,
                    image_type.to_casium_type(),
                ),
            };

            // Handle the result
            if let Err(err) = result {
                fs::remove_file(&temp_path)
                    .map_err(|e| format!("Error removing temp file: {}", e))?;
                return Err(format!("Error: {}", err));
            }

            // Read the temporary file
            let temp_data = fs::read(&temp_path).map_err(|e| format!("Error: {}", e));

            // Clean up temp file
            fs::remove_file(&temp_path).map_err(|e| format!("Error removing temp file: {}", e))?;

            temp_data
        }
    }
    .map_err(|e| format!("Error: {}", e))
}

// Simplified wrapper functions
fn compress_image(original_img_data: Vec<u8>, params: CSParameters) -> Result<Vec<u8>, String> {
    process_image(
        ImageSource::Memory(original_img_data),
        params,
        ImageOperation::Compress,
    )
}

fn convert_image(
    original_img_data: Vec<u8>,
    params: CSParameters,
    image_type: ImageType,
) -> Result<Vec<u8>, String> {
    process_image(
        ImageSource::Memory(original_img_data),
        params,
        ImageOperation::Convert(image_type),
    )
}

fn compress_image_from_file(
    original_img_path: &Path,
    params: CSParameters,
) -> Result<Vec<u8>, String> {
    process_image(
        ImageSource::File(original_img_path.to_path_buf()),
        params,
        ImageOperation::Compress,
    )
}

fn convert_image_from_file(
    original_img_path: &Path,
    params: CSParameters,
    image_type: ImageType,
) -> Result<Vec<u8>, String> {
    process_image(
        ImageSource::File(original_img_path.to_path_buf()),
        params,
        ImageOperation::Convert(image_type),
    )
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
            return Err(format!("Error getting file size: {}", err));
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
    let supported_exts = ["png", "jpeg", "jpg", "gif", "webp", "tiff"];
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

    #[test]
    fn test_get_temp_path() {
        let result = get_temp_path(Path::new("test/test.png"));
        assert_eq!(result, "test/.test.png".to_string());
    }
}
