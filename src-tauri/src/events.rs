use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AddFileEvent(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ClearFilesEvent;

/// A backend error the frontend should surface as a toast.
#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ErrorEvent(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SettingsChangedEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct OpenAddFileDialogEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
#[serde(tag = "type")]
pub enum UpdateStateEvent {
    CheckingForUpdate {
        message: String,
    },
    NoUpdate {
        message: String,
    },
    Error {
        message: String,
    }, // Includes error message
    Downloading {
        percent: f32,
        bytes_downloaded: f32,
        total_bytes: f32,
    },
    Success {
        version: String,
        release_notes: Option<String>,
    },
}
