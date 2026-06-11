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
#[serde(default)]
pub struct SettingsData {
    pub version: u32,
    pub theme: ThemeKind,
    pub threads: i32,
    pub default_profile_id: Option<u32>,
    pub profiles: Vec<ProfileData>,
}

#[derive(serde::Serialize, serde::Deserialize, Type, Debug, Clone, Default)]
pub enum ThemeKind {
    Light,
    Dark,
    #[default]
    System,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            version: 1,
            theme: ThemeKind::System,
            threads: 0,
            default_profile_id: None,
            profiles: vec![ProfileData::default()],
        }
    }
}

impl SettingsData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Type, Debug, Clone)]
#[serde(default)]
pub struct ProfileData {
    pub name: String,
    pub id: u32,
    pub active: bool,
    pub should_resize: bool,
    pub should_convert: bool,
    pub should_overwrite: bool,
    pub enable_lossy: bool,
    pub keep_timestamps: bool,
    pub keep_metadata: bool,
    #[serde(alias = "add_posfix")]
    pub add_postfix: bool,
    pub should_background_fill: bool,
    pub background_fill: String,
    pub convert_extension: ImageType,
    pub postfix: String,
    pub resize_width: u32,
    pub resize_height: u32,
    pub jpeg_quality: u32,
    pub png_quality: u32,
    pub webp_quality: u32,
    pub gif_quality: u32,
    pub avif_quality: u32,
}

fn default_color() -> String {
    "#000".to_string()
}

impl Default for ProfileData {
    fn default() -> Self {
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
}

impl ProfileData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            active: false,
            ..Self::default()
        }
    }
}

#[derive(serde::Serialize, Type, Debug, Clone)]
pub struct SettingsResult {
    pub settings: SettingsData,
    pub warning: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_settings(app: tauri::AppHandle) -> Result<SettingsResult, String> {
    let (settings, warnings) = get_settings_data(&app)?;
    let warning = if warnings.is_empty() {
        None
    } else {
        Some(warnings.join("\n"))
    };
    Ok(SettingsResult { settings, warning })
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
    let mut settings = get_settings_data(&app)?.0;
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
    let mut settings = get_settings_data(&app)?.0;
    settings
        .profiles
        .iter()
        .position(|p| p.id == profile_id)
        .map(|i| settings.profiles.remove(i));
    if settings.default_profile_id == Some(profile_id) {
        settings.default_profile_id = None;
    }
    set_settings_data(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn add_profile(app: tauri::AppHandle, mut name: String) -> Result<(), String> {
    let mut settings = get_settings_data(&app)?.0;
    if settings.profiles.iter().any(|p| p.name == name) {
        let mut n = 2;
        while settings
            .profiles
            .iter()
            .any(|p| p.name == format!("{name} ({n})"))
        {
            n += 1;
        }
        name = format!("{name} ({n})");
    }
    let highest_id = settings.profiles.iter().map(|p| p.id).max().unwrap_or(0);
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

pub fn get_settings_data(app: &tauri::AppHandle) -> Result<(SettingsData, Vec<String>), String> {
    let store = get_store(app);
    let mut warnings = Vec::new();

    let value = match store.get(SETTINGS_KEY) {
        Some(v) => v,
        None => {
            // If the settings file exists on disk but the store has no
            // data, the file was likely corrupt JSON that the store
            // plugin silently discarded.
            let settings_path = app
                .path()
                .app_data_dir()
                .map(|p| p.join("settings.json"))
                .unwrap_or_default();

            let settings = SettingsData::new();
            store.set(SETTINGS_KEY, json!(settings));
            if settings_path.exists() {
                warnings
                    .push("Settings file was corrupt and has been reset to defaults.".to_string());
            }
            return Ok((settings, warnings));
        }
    };

    check_missing_keys(&value, &mut warnings);

    let settings: SettingsData =
        serde_json::from_value(value).map_err(|err| format!("Failed to load settings: {err}"))?;
    if !warnings.is_empty() {
        store.set(SETTINGS_KEY, json!(settings));
    }
    Ok((settings, warnings))
}

pub fn check_missing_keys(value: &serde_json::Value, warnings: &mut Vec<String>) {
    let obj = match value.as_object() {
        Some(o) => o,
        None => return,
    };

    let default_settings = json!(SettingsData::new());
    if let Some(expected) = default_settings.as_object() {
        for (key, default_val) in expected {
            if !obj.contains_key(key) {
                warnings.push(format!(
                    "Missing setting \"{key}\". Setting to {default_val}"
                ));
            }
        }
    }

    let default_profile = json!(ProfileData::default());
    let expected_profile = match default_profile.as_object() {
        Some(o) => o,
        None => return,
    };

    if let Some(profiles) = obj.get("profiles").and_then(|p| p.as_array()) {
        for profile in profiles.iter() {
            if let Some(profile_obj) = profile.as_object() {
                let profile_name = profile_obj
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown");
                for (key, default_val) in expected_profile {
                    if !profile_obj.contains_key(key) {
                        warnings.push(format!(
                            "Profile \"{profile_name}\" missing \"{key}\". Setting to {default_val}"
                        ));
                    }
                }
            }
        }
    }
}

fn set_settings_data(app: &tauri::AppHandle, settings: SettingsData) {
    let store = get_store(app);
    store.set(
        SETTINGS_KEY,
        serde_json::to_value(settings.clone()).unwrap(),
    );
    SettingsChangedEvent.emit(app).unwrap()
}
