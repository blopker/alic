use crate::events::{AddFileEvent, BadFileEvent};
use crate::macos;

use super::settings;
use caesium;
use caesium::parameters::CSParameters;
use image::{self, ImageReader};
use image::{DynamicImage, ImageFormat};
use serde;
use specta::Type;
use std::fs;
use std::os::unix::fs::MetadataExt;
use tauri_plugin_clipboard_manager::ClipboardExt;
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
    app: tauri::AppHandle,
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
    let (original_img, original_image_type) = match read_image(&file.path) {
        Ok(img) => img,
        Err(err) => {
            println!("Error1: {}", err);
            return Err(CompressError {
                error: err,
                error_type: CompressErrorType::UnsupportedFileType,
            });
        }
    };

    let out_path = get_out_path(&parameters, &file.path, &original_image_type);

    if file.path == out_path && !parameters.should_overwrite {
        return Err(CompressError {
            error: "Image would be overwritten. Enable Overwrite in settings to allow this."
                .to_string(),
            error_type: CompressErrorType::WontOverwrite,
        });
    }

    let csparams = create_csparameters(&parameters, original_img.width(), original_img.height());
    // let (o_width, o_height) = (original_img.width(), original_img.height());
    drop(original_img);

    let should_convert =
        parameters.should_convert && parameters.convert_extension != original_image_type;

    let temp_path = get_temp_path(&out_path);
    let result = if should_convert {
        convert_image(
            &file.path,
            &temp_path,
            csparams,
            parameters.convert_extension,
        )
    } else {
        compress_image(&file.path, &temp_path, csparams)
    };

    if result.is_err() {
        return Err(CompressError {
            error: result.err().unwrap().to_string(),
            error_type: CompressErrorType::Unknown,
        });
    }

    let temp_metadata_result = std::fs::metadata(&temp_path);
    let temp_size: f64 = match temp_metadata_result {
        Ok(result) => result.size() as f64,
        Err(e) => {
            println!("Error2: {}", e);
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::FileNotFound,
            });
        }
    };
    let original_size = file.original_size.expect("Image size needs to be set") as f64;

    if !parameters.should_convert && temp_size > original_size * 0.95 {
        let _ = fs::remove_file(temp_path);
        println!("Error3");
        return Err(CompressError {
            error: "Image cannot be compressed further.".to_string(),
            error_type: CompressErrorType::NotSmaller,
        });
    }

    if out_path == file.path {
        let res = macos::trash_file(&file.path);
        if res.is_err() {
            println!("Error4: {}", res.clone().err().unwrap());
            return Err(CompressError {
                error: res.err().unwrap().to_string(),
                error_type: CompressErrorType::Unknown,
            });
        }
    }

    let rename_result = fs::rename(temp_path, &out_path);
    match rename_result {
        Ok(_) => {}
        Err(e) => {
            println!("Error5: {}", e);
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::Unknown,
            });
        }
    };
    let out_size = temp_size as u32;
    // Read the compressed image for clipboard
    match image::open(&out_path) {
        Ok(img) => {
            let rgba_img = img.into_rgba8();
            let width = rgba_img.width();
            let height = rgba_img.height();

            // Ensure we have valid dimensions
            if width == 0 || height == 0 {
                return Err(CompressError {
                    error: "Invalid image dimensions".to_string(),
                    error_type: CompressErrorType::Unknown,
                });
            }

            // Ensure proper byte alignment and create rgba data
            let rgba_data = rgba_img.into_raw();

            // Ensure we have the correct amount of data
            if rgba_data.len() != (width * height * 4) as usize {
                return Err(CompressError {
                    error: "Invalid image data size".to_string(),
                    error_type: CompressErrorType::Unknown,
                });
            }

            // Create tauri image
            let tauri_img = tauri::image::Image::new(&rgba_data, width, height);

            // Try to write to clipboard with error handling
            if let Err(e) = app.clipboard().write_image(&tauri_img) {
                println!("Error6: {}", e);
                return Err(CompressError {
                    error: format!("Failed to copy to clipboard: {}", e),
                    error_type: CompressErrorType::Unknown,
                });
            }
        }
        Err(e) => {
            return Err(CompressError {
                error: format!("Failed to read image for clipboard: {}", e),
                error_type: CompressErrorType::Unknown,
            })
        }
    }

    Ok(CompressResult {
        path: file.path,
        out_size,
        out_path,
        result: "Success".to_string(),
    })
}

fn get_temp_path(path: &str) -> String {
    // /original/path/test.png -> /original/path/.test.png
    let path = Path::new(&path);
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    let directory = path.parent().unwrap().to_string_lossy().to_string();
    Path::new(&directory)
        .join(format!(".{}", filename))
        .to_string_lossy()
        .to_string()
}

fn read_image(path: &str) -> Result<(DynamicImage, ImageType), String> {
    let image = ImageReader::open(&path)
        .map_err(|e| e.to_string())?
        .with_guessed_format();

    let format = match &image {
        Ok(image) => match image.format() {
            Some(ImageFormat::Jpeg) => Some(ImageType::JPEG),
            Some(ImageFormat::Png) => Some(ImageType::PNG),
            Some(ImageFormat::WebP) => Some(ImageType::WEBP),
            Some(ImageFormat::Gif) => Some(ImageType::GIF),
            Some(ImageFormat::Tiff) => Some(ImageType::TIFF),
            f => {
                return Err(format!(
                    "Error: Unsupported image type: {}",
                    f.unwrap().to_mime_type()
                ))
            }
        },
        Err(_) => None,
    };

    if format.is_none() {
        return Err("Error: Unsupported image type.".to_string());
    }

    match image {
        Ok(image) => Ok((image.decode().map_err(|e| e.to_string())?, format.unwrap())),
        Err(err) => Err(format!("Read error: {}", err)),
    }
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
    let posfix = match parameters.add_posfix {
        true => parameters.postfix.clone(),
        false => "".to_string(),
    };
    format!("{}{}.{}", remove_extension(&path), posfix, extension)
}

fn create_csparameters(
    parameters: &settings::ProfileData,
    width: u32,
    height: u32,
) -> CSParameters {
    let mut new_height = 0;
    let mut new_width = 0;

    // set the largest dimension to the resize value,
    // only if the image size is larger than the resize value
    if parameters.should_resize {
        if width > parameters.resize_width || height > parameters.resize_height {
            if width > height {
                new_width = parameters.resize_width;
            } else {
                new_height = parameters.resize_height;
            }
        }
    }

    let mut cspars = CSParameters::new();
    cspars.jpeg.quality = parameters.jpeg_quality;
    cspars.png.quality = parameters.png_quality;
    cspars.webp.quality = parameters.webp_quality;
    cspars.gif.quality = parameters.gif_quality;
    cspars.width = new_width;
    cspars.height = new_height;
    cspars.optimize = !parameters.enable_lossy;
    cspars.keep_metadata = parameters.keep_metadata;
    cspars
}

fn compress_image(path: &str, out_path: &str, mut params: CSParameters) -> Result<String, String> {
    let result = caesium::compress(path.to_string(), out_path.to_string(), &mut params);
    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(err) => Err(format!("Error: {}", err)),
    }
}

fn convert_image(
    path: &str,
    out_path: &str,
    mut params: CSParameters,
    image_type: ImageType,
) -> Result<String, String> {
    let result = caesium::convert(
        path.to_string(),
        out_path.to_string(),
        &mut params,
        image_type.to_casium_type(),
    );

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(err) => Err(format!("Error: {}", err)),
    }
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
        if !is_image(&file) {
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
    let metadata_result = std::fs::metadata(&path);
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

pub fn is_image<P: AsRef<Path>>(path: &P) -> bool {
    let path = path.as_ref();
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
        let mut result = get_out_path(&parameters, &"test/test.png".to_string(), &ImageType::PNG);
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        result = get_out_path(&parameters, &"test/test.jpeg".to_string(), &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        result = get_out_path(&parameters, &"test/test.jpg".to_string(), &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpg".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = true;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string(), &ImageType::JPEG);
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = false;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string(), &ImageType::JPEG);
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        parameters.add_posfix = false;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string(), &ImageType::PNG);
        assert_eq!(result, "test/test.png".to_string());

        parameters = settings::ProfileData::new();
        parameters.postfix = ".bong".to_string();
        result = get_out_path(&parameters, &"test/test.jpeg".to_string(), &ImageType::PNG);
        assert_eq!(result, "test/test.bong.png".to_string());
    }

    #[test]
    fn test_get_temp_path() {
        let result = get_temp_path(&"test/test.png".to_string());
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
