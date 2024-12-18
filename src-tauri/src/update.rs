use crate::events::UpdateStateEvent;
use tauri_plugin_updater::UpdaterExt;
use tauri_specta::Event;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    match app.updater()?.check().await? {
        Some(update) => {
            let mut downloaded = 0;
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        println!("downloaded {downloaded} from {content_length:?}");
                        let mut percent = 0.0;
                        let mut total_bytes = 0.0;
                        if content_length.is_some() {
                            percent = (downloaded as f32 / content_length.unwrap() as f32) * 100.0;
                            total_bytes = content_length.unwrap() as f32;
                        }
                        UpdateStateEvent::Downloading {
                            percent,
                            bytes_downloaded: downloaded as f32,
                            total_bytes,
                        }
                        .emit(&app)
                        .unwrap();
                    },
                    || {
                        println!("download finished");
                    },
                )
                .await?;
            UpdateStateEvent::Success {
                version: update.version,
                release_notes: update.body,
            }
            .emit(&app)
            .unwrap();
            println!("update installed");
            app.restart();
        }
        None => {
            UpdateStateEvent::NoUpdate {
                message: "No update available".to_string(),
            }
            .emit(&app)
            .unwrap();
            println!("no update available");
        }
    };

    Ok(())
}
