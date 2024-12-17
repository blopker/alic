use crate::events::emit_add_file;
use crate::macos;

use super::settings;
use caesium;
use caesium::parameters::CSParameters;
use image;
use image::DynamicImage;
use serde;
use specta::Type;
use std::fs;
use std::os::unix::fs::MetadataExt;

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

#[derive(Debug, Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
pub enum ImageType {
    JPEG,
    PNG,
    WEBP,
    GIF,
    TIFF,
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
    let out_path = get_out_path(&parameters, &file.path);

    if file.path == out_path && !parameters.should_overwrite {
        return Err(CompressError {
            error: "Image would be overwritten. Enable Overwrite in settings to allow this."
                .to_string(),
            error_type: CompressErrorType::WontOverwrite,
        });
    }

    let original_img = match read_image(&file.path) {
        Ok(img) => img,
        Err(err) => {
            return Err(CompressError {
                error: err,
                error_type: CompressErrorType::UnsupportedFileType,
            })
        }
    };

    let csparams = create_csparameters(&parameters, original_img.width(), original_img.height());
    drop(original_img);

    let original_image_type = match guess_image_type(&file.path) {
        Ok(img) => img,
        Err(err) => {
            return Err(CompressError {
                error: err,
                error_type: CompressErrorType::UnsupportedFileType,
            })
        }
    };
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
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::FileNotFound,
            })
        }
    };
    let original_size = file.original_size.expect("Image size needs to be set") as f64;

    if !parameters.should_convert && temp_size > original_size * 0.95 {
        let _ = fs::remove_file(temp_path);
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

    let rename_result = fs::rename(temp_path, &out_path);
    match rename_result {
        Ok(_) => {}
        Err(e) => {
            return Err(CompressError {
                error: e.to_string(),
                error_type: CompressErrorType::Unknown,
            })
        }
    };
    let out_size = temp_size as u32;
    Ok(CompressResult {
        path: file.path,
        out_size,
        out_path,
        result: "Success".to_string(),
    })
}

// #[derive(Debug)]
// struct ScanError {
//     error: String,
//     path: String,
// }

// pub fn scan_directory_for_images(
//     path: impl Into<PathBuf>,
// ) -> impl Stream<Item = Result<PathBuf, ScanError>> {
//     let (tx, rx) = mpsc::channel(100);
//     let path = path.into();
//     if !path.is_dir() {
//         if is_image(&path) {
//             tx.send(Ok(path));
//         } else {
//             tx.send(Err(ScanError {
//                 path: path.to_string_lossy().to_string(),
//                 error: "Not a directory or image".to_string(),
//             }));
//         }
//         return ReceiverStream::new(rx);
//     }
//     tokio::spawn(async move {
//         async fn visit_dirs(
//             dir: PathBuf,
//             tx: mpsc::Sender<Result<PathBuf, ScanError>>,
//         ) -> Result<(), String> {
//             let read_dir = tokio::fs::read_dir(&dir).await.map_err(|e| e.to_string())?;

//             let mut read_dir = read_dir;
//             while let Some(entry) = read_dir.next_entry().await.map_err(|e| e.to_string())? {
//                 let path = entry.path();
//                 if is_image(&path) {
//                     if tx.send(Ok(path)).await.is_err() {
//                         break;
//                     }
//                 } else if path.is_dir() {
//                     let future = visit_dirs(path.clone(), tx.clone());
//                     if let Err(e) = Box::pin(future).await {
//                         let _ = tx
//                             .send(Err(ScanError {
//                                 path: path.to_string_lossy().to_string(),
//                                 error: e,
//                             }))
//                             .await;
//                     }
//                 }
//             }
//             Ok(())
//         }

//         if let Err(e) = Box::pin(visit_dirs(path.clone(), tx.clone())).await {
//             let _ = tx
//                 .send(Err(ScanError {
//                     path: path.to_string_lossy().to_string(),
//                     error: e,
//                 }))
//                 .await;
//         }
//     });

//     ReceiverStream::new(rx)
// }

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

fn read_image(path: &str) -> Result<DynamicImage, String> {
    let image = image::open(&path);
    match image {
        Ok(image) => Ok(image),
        Err(err) => Err(format!("Error: {}", err)),
    }
}

fn guess_image_type(path: &str) -> Result<ImageType, String> {
    let kind =
        infer::get_from_path(path).map_err(|e| format!("Error determining file type: {}", e))?;
    match kind {
        Some(kind) => match kind.mime_type() {
            "image/jpeg" => Ok(ImageType::JPEG),
            "image/png" => Ok(ImageType::PNG),
            "image/webp" => Ok(ImageType::WEBP),
            "image/gif" => Ok(ImageType::GIF),
            "image/tiff" => Ok(ImageType::TIFF),
            _ => Err(format!(
                "Error: Unsupported image type: {}",
                kind.mime_type()
            )),
        },
        None => Err("Error: Could not determine image type.".to_string()),
    }
}

fn get_out_path(parameters: &settings::ProfileData, path: &str) -> String {
    let path = Path::new(&path);
    let extension = match parameters.should_convert {
        true => image_type_to_extension(parameters.convert_extension),
        false => path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    };
    let posfix = match parameters.add_posfix {
        true => parameters.postfix.clone(),
        false => "".to_string(),
    };
    format!("{}{}.{}", remove_extension(&path), posfix, extension)
}

fn image_type_to_extension(image_type: ImageType) -> String {
    match image_type {
        ImageType::JPEG => "jpg".to_string(),
        ImageType::PNG => "png".to_string(),
        ImageType::WEBP => "webp".to_string(),
        ImageType::GIF => "gif".to_string(),
        ImageType::TIFF => "tiff".to_string(),
    }
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
    let supported_type = match image_type {
        ImageType::JPEG => caesium::SupportedFileTypes::Jpeg,
        ImageType::PNG => caesium::SupportedFileTypes::Png,
        ImageType::WEBP => caesium::SupportedFileTypes::WebP,
        ImageType::GIF => caesium::SupportedFileTypes::Gif,
        ImageType::TIFF => caesium::SupportedFileTypes::Tiff,
    };
    let result = caesium::convert(
        path.to_string(),
        out_path.to_string(),
        &mut params,
        supported_type,
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
    let on_event = |path: String| emit_add_file(&app, path);

    let file = Path::new(&path);
    if !file.exists() {
        return Ok(());
    }
    if file.is_file() {
        if !is_image(&file) {
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
        let result = image_type_to_extension(ImageType::JPEG);
        assert_eq!(result, "jpg".to_string());
    }

    #[test]
    fn test_guess_image_type() {
        let result = guess_image_type("test/test.jpg");
        assert_eq!(result.unwrap(), ImageType::JPEG);
    }

    #[test]
    fn test_get_out_path() {
        let mut parameters = settings::ProfileData::new();
        let mut result = get_out_path(&parameters, &"test/test.png".to_string());
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        result = get_out_path(&parameters, &"test/test.jpeg".to_string());
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = true;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string());
        assert_eq!(result, "test/test.min.png".to_string());

        parameters = settings::ProfileData::new();
        parameters.should_convert = false;
        parameters.convert_extension = ImageType::PNG;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string());
        assert_eq!(result, "test/test.min.jpeg".to_string());

        parameters = settings::ProfileData::new();
        parameters.add_posfix = false;
        result = get_out_path(&parameters, &"test/test.jpeg".to_string());
        assert_eq!(result, "test/test.jpeg".to_string());

        parameters = settings::ProfileData::new();
        parameters.postfix = ".bong".to_string();
        result = get_out_path(&parameters, &"test/test.jpeg".to_string());
        assert_eq!(result, "test/test.bong.jpeg".to_string());
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
