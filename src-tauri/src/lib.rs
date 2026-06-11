pub(crate) mod cli;
pub mod compress;
pub(crate) mod errors;
mod events;
mod macos;
mod resize;
pub mod settings;
mod update;

use events::{AddFileEvent, ClearFilesEvent, ErrorEvent, OpenAddFileDialogEvent, UpdateStateEvent};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
};
use tauri::{
    Manager, WebviewUrl,
    menu::{AboutMetadataBuilder, Menu, MenuItem, SubmenuBuilder},
    utils::config::WindowConfig,
};
use tauri_plugin_cli::CliExt;
use tauri_plugin_opener::OpenerExt;

use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_store::StoreExt;
use tauri_specta::{Builder, Event, collect_commands, collect_events};

#[tauri::command]
#[specta::specta]
async fn open_settings_window(app: tauri::AppHandle, path: Option<String>) -> Result<(), String> {
    _open_settings_window(&app, path)
}

#[tauri::command]
#[specta::specta]
async fn open_link_in_browser(app: tauri::AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open link: {e}"))
}

fn _open_settings_window(app: &tauri::AppHandle, path: Option<String>) -> Result<(), String> {
    let window_label = "settings";
    let config = app
        .config()
        .app
        .windows
        .iter()
        .find(|w| w.label == window_label)
        .ok_or("Settings window configuration not found")?;
    let path = path.unwrap_or("/settings".to_string());
    if let Some(window) = app.get_webview_window(window_label) {
        // If the window already exists, bring it to the front
        let mut url = window.url().map_err(|e| e.to_string())?;
        url.set_fragment(Some(&path));
        window.navigate(url).map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
    } else {
        // If the window does not exist, create it
        let url = format!("/index.html#{path}");
        let new_conf = WindowConfig {
            url: WebviewUrl::App(Path::new(&url).to_path_buf()),
            ..config.clone()
        };
        tauri::WebviewWindowBuilder::from_config(app, &new_conf)
            .map_err(|e| e.to_string())?
            .build()
            .map_err(|e| e.to_string())?;
    };
    Ok(())
}

fn open_settings_window_or_toast(app: &tauri::AppHandle, path: Option<String>) {
    if let Err(err) = _open_settings_window(app, path) {
        log::error!("Failed to open settings window: {err}");
        let _ = ErrorEvent(format!("Failed to open settings: {err}")).emit(app);
    }
}

fn save_clipboard_image(app: &tauri::AppHandle) -> Result<(), String> {
    // No image on the clipboard (e.g. a plain text paste) is not an error
    let Ok(clip_image) = app.clipboard().read_image() else {
        log::debug!("clip: no image in clipboard");
        return Ok(());
    };

    // If there is also text, this is probably a file copy. Not supported.
    if let Ok(text) = app.clipboard().read_text() {
        log::debug!("clip: clipboard has text, probably a file copy. Not supported: {text:?}");
        return Ok(());
    }

    let image = image::RgbaImage::from_raw(
        clip_image.width(),
        clip_image.height(),
        clip_image.rgba().into(),
    )
    .ok_or("Invalid image data in clipboard")?;
    let mut hasher = DefaultHasher::new();
    clip_image.rgba().hash(&mut hasher);
    let h = hasher.finish();
    // put in $HOME/Documents/alic
    let home =
        std::env::var("HOME").map_err(|e| format!("Could not find home directory: {e}"))?;
    let dir = format!("{home}/Documents/alic");
    // ensure dir exists
    std::fs::create_dir_all(&dir).map_err(|e| format!("Could not create {dir}: {e}"))?;
    let path = format!("{dir}/{h}.png");
    image
        .save(&path)
        .map_err(|e| format!("Could not save image to {path}: {e}"))?;
    AddFileEvent(path).emit(app).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Handle --help and --version before launching the Tauri runtime,
    // since the CLI plugin suppresses clap's default exit behavior.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        cli::print_help();
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            open_settings_window,
            open_link_in_browser,
            compress::process_img,
            compress::get_file_info,
            compress::get_all_images,
            settings::get_settings,
            settings::save_settings,
            settings::reset_settings,
            settings::reset_profile,
            settings::delete_profile,
            settings::add_profile,
            settings::open_settings_folder,
            macos::open_finder_at_path,
            macos::get_cpu_count,
            macos::get_accent_color,
            macos::set_dock_badge,
            macos::bounce_dock_icon,
        ])
        .events(collect_events![
            events::AddFileEvent,
            events::BadFileEvent,
            events::ClearFilesEvent,
            events::ErrorEvent,
            events::SettingsChangedEvent,
            events::OpenAddFileDialogEvent,
            events::UpdateStateEvent,
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log_level)
                .clear_targets()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            println!("Second instance detected:");
            // println!("App: {:?}", app.cli().matches());
            // println!("CWD: {:?}", cwd);
            // println!("Args: {:?}", args);
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .menu(|app| {
            let menu = Menu::new(app)?;
            let about = AboutMetadataBuilder::default()
                .copyright(Some("© All rights reserved Bollc."))
                .build();
            let submenu = SubmenuBuilder::new(app, "Main")
                .about(Some(about))
                .item(&MenuItem::with_id(
                    app,
                    "update_check",
                    "Check for Updates",
                    true,
                    None::<&str>,
                )?)
                .item(&MenuItem::with_id(
                    app,
                    "settings",
                    "Settings...",
                    true,
                    Some("CmdOrCtrl+,"),
                )?)
                .item(&MenuItem::with_id(
                    app,
                    "newprofile",
                    "New Profile...",
                    true,
                    None::<&str>,
                )?)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .quit()
                .build()?;
            menu.append(&submenu)?;
            // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
            let file_submenu = SubmenuBuilder::new(app, "File")
                .item(&MenuItem::with_id(
                    app,
                    "open",
                    "Open Image...",
                    true,
                    Some("CmdOrCtrl+O"),
                )?)
                .item(&MenuItem::with_id(
                    app,
                    "clear",
                    "Clear Images",
                    true,
                    Some("CmdOrCtrl+D"),
                )?)
                // Doesn't work with file paste, just image data
                .item(&MenuItem::with_id(
                    app,
                    "paste",
                    "Paste Images",
                    true,
                    Some("CmdOrCtrl+V"),
                )?)
                .close_window()
                .build()?;
            menu.append(&file_submenu)?;
            Ok(menu)
        })
        .on_menu_event(|app, event| match event.id().0.as_str() {
            "settings" => {
                open_settings_window_or_toast(app, None);
            }
            "newprofile" => {
                open_settings_window_or_toast(app, Some("/settings/newprofile".to_string()));
            }
            "open" => OpenAddFileDialogEvent.emit(app).unwrap(),
            "clear" => ClearFilesEvent.emit(app).unwrap(),
            "paste" => {
                if let Err(err) = save_clipboard_image(app) {
                    log::error!("Paste failed: {err}");
                    let _ = ErrorEvent(format!("Paste failed: {err}")).emit(app);
                }
            }
            "update_check" => {
                println!("Checking for updates");
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    UpdateStateEvent::CheckingForUpdate {
                        message: "Checking for updates...".to_string(),
                    }
                    .emit(&handle)
                    .unwrap();
                    let res = update::update(handle.clone()).await;
                    if let Err(err) = res {
                        println!("Error checking for updates: {err:?}");
                        UpdateStateEvent::Error {
                            message: format!("{err}"),
                        }
                        .emit(&handle)
                        .unwrap();
                    }
                });
            }
            _ => {}
        })
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required for events
            builder.mount_events(app);
            // Store setup
            app.store("settings.json")?;

            // Handle CLI args — if --input was provided, process and exit
            if let Ok(matches) = app.cli().matches() {
                cli::handle_matches(app.handle(), matches);
            }

            // After CLI handling so CLI runs keep using the last active
            // profile rather than the startup default
            settings::activate_startup_profile(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
