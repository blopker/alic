use crate::events::{AddFileEvent, BadFileEvent};
use crate::macos;

use super::settings;
use caesium::parameters::CSParameters;
use image::ImageFormat;
use image::{self, ImageReader};
use specta::Type;
use std::fs;
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
    let (original_img_width, original_img_height, original_image_type) =
        match read_image_info(&file.path) {
            Ok(img) => img,
            Err(err) => {
                return Err(CompressError {
                    error: err,
                    error_type: CompressErrorType::UnsupportedFileType,
                });
            }
        };

    let out_path = get_out_path(&parameters, &file.path, &original_image_type);

    if file.path == out_path && !parameters.should_overwrite {
        return Err(CompressError {
            error:
                "Image would be overwritten. Enable \"Allow Overwrite\" in settings to allow this."
                    .to_string(),
            error_type: CompressErrorType::WontOverwrite,
        });
    }

    let cs_params = create_cs_parameters(&parameters, original_img_width, original_img_height);

    let should_convert =
        parameters.should_convert && parameters.convert_extension != original_image_type;

    let original_img_data = fs::read(&file.path).unwrap();
    let original_img_size = original_img_data.len() as f64;
    let result = match original_image_type {
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
                convert_image(original_img_data, cs_params, parameters.convert_extension)
            } else {
                compress_image(original_img_data, cs_params)
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
    if !parameters.should_convert && compressed_size > original_img_size * 0.95 {
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

    let write_result = fs::write(&out_path, &compressed_data);
    match write_result {
        Ok(_) => {}
        Err(e) => {
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::Unknown,
            });
        }
    };

    let out_size = compressed_data.len() as u32;
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

fn read_image_info(path: &str) -> Result<(u32, u32, ImageType), String> {
    let image = match ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .with_guessed_format()
    {
        Ok(image) => image,
        Err(err) => {
            return Err(format!("Read error: {}", err));
        }
    };

    let format = match image.format() {
        Some(ImageFormat::Jpeg) => Some(ImageType::JPEG),
        Some(ImageFormat::Png) => Some(ImageType::PNG),
        Some(ImageFormat::WebP) => Some(ImageType::WEBP),
        Some(ImageFormat::Gif) => Some(ImageType::GIF),
        Some(ImageFormat::Tiff) => Some(ImageType::TIFF),
        f => {
            return Err(format!(
                "Error: Unsupported image type: {}",
                f.unwrap().to_mime_type()
            ));
        }
    };

    if format.is_none() {
        return Err("Error: Unsupported image type.".to_string());
    }

    let decoded_image = match image.decode() {
        Ok(image) => image,
        Err(err) => {
            return Err(format!("Decode error: {}", err));
        }
    };

    Ok((
        decoded_image.width(),
        decoded_image.height(),
        format.unwrap(),
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
    // #[test]
    // fn test_process_image() {
    //     let parameters = Parameters {
    //         path: "test/test.png".to_string(),
    //         postfix: ".min".to_string(),
    //         resize: true,
    //         resize_width: 1000,
    //         resize_height: 1000,
    //         jpeg_quality: 80,
    //         png_quality: 80,
    //         webp_quality: 80,
    //         gif_quality: 80,
    //         convert_extension: None,
    //     };
    //     let result = process_img(parameters).await;
    //     assert_eq!(result.result, "Success".to_string());
    //     assert_eq!(result.out_path, "test/test.min.png".to_string());
    // }
    // #[test]
    // fn test_garbage() {
    //     unsafe {
    //         let mut num_cores = 0;
    //         let mut len = mem::size_of::<libc::size_t>() as libc::size_t;
    //         libc::sysctlbyname(
    //             "hw.ncpu\0".as_ptr() as *const i8,
    //             &mut num_cores as *mut _ as *mut libc::c_void,
    //             &mut len,
    //             core::ptr::null_mut(),
    //             0,
    //         );
    //         println!("Number of cores: {}", num_cores);
    //     }
    //     assert!(false);
    // }
}
