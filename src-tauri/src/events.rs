use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AddFileEvent(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ClearFilesEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SettingsChangedEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct OpenAddFileDialogEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct UpdateResultsEvent {
    pub(crate) new_version: String,
    pub(crate) update_available: bool,
    pub(crate) download_url: String,
    pub(crate) error: Option<String>,
}

// pub fn emit_add_file(app: &tauri::AppHandle, path: String) {
//     // app.emit("add-file", path).unwrap()
//     AddFileEvent(path).emit(app).unwrap()
// }

// pub fn emit_clear_files(app: &tauri::AppHandle) {
//     // app.emit("clear-files", ()).unwrap()
//     ClearFilesEvent.emit(app).unwrap()
// }

// pub fn emit_settings_changed(app: &tauri::AppHandle) {
//     // app.emit("settings-changed", ()).unwrap()
//     SettingsChangedEvent.emit(app).unwrap()
// }

// pub fn emit_open_add_file_dialog(app: &tauri::AppHandle) {
//     // app.emit("open-add-file-dialog", ()).unwrap()
//     OpenAddFileDialogEvent.emit(app).unwrap()
// }

// pub fn emit_update_results(
//     app: &tauri::AppHandle,
//     new_version: String,
//     message: String,
//     download_url: String,
// ) {
//     // app.emit("update-results", result).unwrap()
//     UpdateResultsEvent {
//         new_version,
//         message,
//         download_url,
//         error: None,
//     }
//     .emit(app)
//     .unwrap()
// }
