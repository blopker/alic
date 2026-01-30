use crate::events::SettingsChangedEvent;
use serde::{self};
use serde_json::json;
use specta::Type;
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tauri_specta::Event;

use crate::compress::ImageType;

const SETTINGS_KEY: &str = "settings";

#[derive(serde::Serialize, serde::Deserialize, Type, Debug, Clone)]
pub struct SettingsData {
    pub version: u32,
    pub theme: ThemeKind,
    #[serde(default)]
    pub threads: i32,
    pub profiles: Vec<ProfileData>,
}

#[derive(serde::Serialize, serde::Deserialize, Type, Debug, Clone)]
pub enum ThemeKind {
    Light,
    Dark,
    System,
}

impl SettingsData {
    pub fn new() -> Self {
        Self {
            version: 1,
            theme: ThemeKind::System,
            threads: 0,
            profiles: vec![ProfileData::new()],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Type, Debug, Clone)]
pub struct ProfileData {
    pub name: String,
    pub id: u32,
    pub active: bool,
    pub should_resize: bool,
    pub should_convert: bool,
    pub should_overwrite: bool,
    #[serde(default)]
    pub enable_lossy: bool,
    #[serde(default)]
    pub keep_timestamps: bool,
    #[serde(default)]
    pub keep_metadata: bool,
    #[serde(default, alias = "add_posfix")]
    pub add_postfix: bool,
    #[serde(default)]
    pub should_background_fill: bool,
    #[serde(default = "default_color")]
    pub background_fill: String,
    pub convert_extension: ImageType,
    pub postfix: String,
    pub resize_width: u32,
    pub resize_height: u32,
    pub jpeg_quality: u32,
    pub png_quality: u32,
    pub webp_quality: u32,
    pub gif_quality: u32,
    #[serde(default = "default_avif_quality")]
    pub avif_quality: u32,
}

fn default_avif_quality() -> u32 {
    80
}

fn default_color() -> String {
    "#000".to_string()
}

impl ProfileData {
    pub fn new() -> Self {
        Self {
            name: "Default".to_string(),
            id: 0,
            active: true,
            should_resize: false,
            should_background_fill: false,
            background_fill: default_color(),
            should_convert: false,
            should_overwrite: false,
            enable_lossy: true,
            keep_timestamps: false,
            keep_metadata: true,
            add_postfix: true,
            convert_extension: ImageType::WEBP,
            postfix: ".min".to_string(),
            resize_width: 1000,
            resize_height: 1000,
            jpeg_quality: 80,
            png_quality: 80,
            webp_quality: 80,
            gif_quality: 80,
            avif_quality: 80,
        }
    }

    pub fn new_with_params(id: u32, name: String) -> Self {
        let mut this = Self::new();
        this.id = id;
        this.name = name;
        this.active = false;
        this
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_settings(app: tauri::AppHandle) -> Result<SettingsData, String> {
    Ok(get_settings_data(&app))
}

#[tauri::command]
#[specta::specta]
pub async fn save_settings(app: tauri::AppHandle, settings: SettingsData) -> Result<(), String> {
    set_settings_data(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn reset_settings(app: tauri::AppHandle) -> Result<(), String> {
    set_settings_data(&app, SettingsData::new());
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn reset_profile(app: tauri::AppHandle, profile_id: u32) -> Result<(), String> {
    let mut settings = get_settings_data(&app);
    let profile_idx = settings.profiles.iter().position(|p| p.id == profile_id);
    if profile_idx.is_none() {
        return Err("Profile not found".to_string());
    }
    let profile = settings.profiles[profile_idx.unwrap()].clone();
    settings.profiles[profile_idx.unwrap()] =
        ProfileData::new_with_params(profile_id, profile.name);
    set_settings_data(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_profile(app: tauri::AppHandle, profile_id: u32) -> Result<(), String> {
    if profile_id == 0 {
        return Err("Cannot delete default profile".to_string());
    }
    let mut settings = get_settings_data(&app);
    settings
        .profiles
        .iter()
        .position(|p| p.id == profile_id)
        .map(|i| settings.profiles.remove(i));
    set_settings_data(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn add_profile(app: tauri::AppHandle, mut name: String) -> Result<(), String> {
    let mut settings = get_settings_data(&app);
    let profile_idx = settings.profiles.iter().position(|p| p.name == name);
    if let Some(id) = profile_idx {
        name = format!("{} ({})", name, id + 1);
    }
    let highest_id = settings.profiles.iter().max_by_key(|p| p.id).unwrap().id;
    settings
        .profiles
        .push(ProfileData::new_with_params(highest_id + 1, name));
    set_settings_data(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn open_settings_folder(_app: tauri::AppHandle) -> Result<(), String> {
    let parent_dir = _app.path().app_data_dir().unwrap();
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
    }

    Ok(())
}

fn get_store(app: &tauri::AppHandle) -> Arc<Store<Wry>> {
    app.store("settings.json")
        .expect("Failed to get settings from store")
}

pub fn get_settings_data(app: &tauri::AppHandle) -> SettingsData {
    let store = get_store(app);
    let settings: Option<SettingsData> = store
        .get(SETTINGS_KEY)
        .and_then(|v| serde_json::from_value(v).ok());

    if settings.is_none() {
        let settings = SettingsData::new();
        store.set(SETTINGS_KEY, json!(settings));
        return settings;
    }

    settings.unwrap()
}

fn set_settings_data(app: &tauri::AppHandle, settings: SettingsData) {
    let store = get_store(app);
    store.set(
        SETTINGS_KEY,
        serde_json::to_value(settings.clone()).unwrap(),
    );
    SettingsChangedEvent.emit(app).unwrap()
}
