use specta::Type;

#[derive(serde::Serialize, serde::Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlicError {
    pub error: String,
    pub error_type: AlicErrorType,
}

#[derive(serde::Serialize, serde::Deserialize, Type)]
pub enum AlicErrorType {
    Unknown,
    FileTooLarge,
    FileNotFound,
    UnsupportedFileType,
    WontOverwrite,
    NotSmaller,
    ImageResizeError,
    InvalidHexColor,
}
