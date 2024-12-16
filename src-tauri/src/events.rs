use tauri::Emitter;

pub fn emit_add_file(app: &tauri::AppHandle, path: String) {
    app.emit("add-file", path).unwrap()
}

pub fn emit_clear_files(app: &tauri::AppHandle) {
    app.emit("clear-files", ()).unwrap()
}

pub fn emit_settings_changed(app: &tauri::AppHandle) {
    app.emit("settings-changed", ()).unwrap()
}

pub fn emit_open_add_file_dialog(app: &tauri::AppHandle) {
    app.emit("open-add-file-dialog", ()).unwrap()
}
