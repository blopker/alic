mod compress;
mod events;
mod macos;
mod settings;
mod update;

use compress::is_image;
use events::{AddFileEvent, ClearFilesEvent, OpenAddFileDialogEvent, UpdateStateEvent};
use image;
use infer::text;
use std::{
    cell::OnceCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
    sync::{Mutex, OnceLock},
    time::Instant,
};
use tauri::{
    menu::{AboutMetadataBuilder, Menu, MenuItem, SubmenuBuilder},
    utils::config::WindowConfig,
    Listener, Manager, WebviewUrl,
};
use tauri_plugin_opener::OpenerExt;

use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_store::StoreExt;
use tauri_specta::{collect_commands, collect_events, Builder, Event};

#[tauri::command]
#[specta::specta]
async fn open_settings_window(app: tauri::AppHandle, path: Option<String>) {
    _open_settings_window(&app, path);
}

#[tauri::command]
#[specta::specta]
async fn open_link_in_browser(app: tauri::AppHandle, url: String) {
    app.opener().open_url(url, None::<&str>).unwrap();
}

fn _open_settings_window(app: &tauri::AppHandle, path: Option<String>) {
    let window_label = "settings";
    let config = &app
        .config()
        .app
        .windows
        .iter()
        .find(|w| w.label == window_label)
        .unwrap();
    let path = path.unwrap_or("/settings".to_string());
    if let Some(mut window) = app.get_webview_window(window_label) {
        // If the window already exists, bring it to the front
        let mut url = window.url().unwrap();
        url.set_fragment(Some(&path));
        window.navigate(url).unwrap();
        window.show().unwrap();
    } else {
        // If the window does not exist, create it
        let url = format!("/index.html#{path}");
        let newconf = WindowConfig {
            url: WebviewUrl::App(Path::new(&url).to_path_buf()),
            ..(*config).clone().to_owned()
        };
        tauri::WebviewWindowBuilder::from_config(app, &newconf)
            .unwrap()
            .build()
            .unwrap();
    };
}

// static LAST_IMAGE_HASH: OnceLock<Mutex<Option<u64>>> = OnceLock::new();
static LAST_TIMESTAMP: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();

fn save_clipboard_image(app: &tauri::AppHandle) {
    let clipboard = app.state::<tauri_plugin_clipboard::Clipboard>();
    // Try reading image
    let image_result = clipboard.read_image_binary();
    // let image_result = app.clipboard().read_image();
    if image_result.is_err() {
        println!("clip: no image {:?}", image_result);
        return;
    }
    let mut clip_image: Option<Vec<u8>>;
    clip_image = image_result.ok();
    let now = Instant::now();
    if clip_image.is_none() {
        let Ok(file_results) = clipboard.read_files() else {
            println!("Read file failed");
            return;
        };

        match file_results.as_slice() {
            [first, ..] => {
                let path = Path::new(first);
                if is_image(&path) {
                    clip_image = std::fs::read(path)
                        .map_err(|e| {
                            println!("err: {e}");
                            e
                        })
                        .ok();
                }
            }
            _ => return,
        }
    }

    let Some(clip_image) = clip_image else {
        return;
    };

    // // Read filename
    // let Ok(file_name) = app.clipboard().read_text() else {
    //     return;
    // };
    // if ["png", "jpeg", "jpg", "gif", "webp", "tiff"]

    // println!(
    //     "Not just image data, probably a file copy. Not supported: {:?}",
    //     text_result
    // );
    // return;

    // get nicely printed current time to the second, with no external dependencies
    // let image = image::RgbaImage::from_raw(
    //     clip_image.width(),
    //     clip_image.height(),
    //     clip_image.rgba().into(),
    // )
    // .unwrap();
    let image = image::load_from_memory(&clip_image).unwrap();
    let mut hasher = DefaultHasher::new();
    image.to_rgb8().hash(&mut hasher);
    let h = hasher.finish();
    let mut guard = LAST_TIMESTAMP
        .get_or_init(|| Mutex::new(None))
        .lock()
        .expect("Mutex poisoned");
    if let Some(prev_timestamp) = *guard {
        if now.duration_since(prev_timestamp) < std::time::Duration::from_millis(5000) {
            println!("Skipping repeated image");
            return;
        }
        println!("now: {:?}", now);
    }
    // Update the stored hash
    *guard = Some(now);
    drop(guard);

    // put in $HOME/Documents/alic
    let dir = format!("{}/Downloads/alic", std::env::var("HOME").unwrap());
    // ensure dir exists
    std::fs::create_dir_all(&dir).unwrap();
    let path = format!("{dir}/{}.png", h);
    image.save(&path).unwrap();
    AddFileEvent(path).emit(app).unwrap();
    // app.clipboard().write_image();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            macos::open_finder_at_path,
            macos::get_cpu_count,
        ])
        .events(collect_events![
            events::AddFileEvent,
            events::BadFileEvent,
            events::ClearFilesEvent,
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
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            println!("Second instance detected:");
            // println!("App: {:?}", app.cli().matches());
            // println!("CWD: {:?}", cwd);
            // println!("Args: {:?}", args);
        }))
        .plugin(tauri_plugin_deep_link::init())
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
        .on_menu_event(|app, event| {
            return match event.id().0.as_str() {
                "settings" => {
                    _open_settings_window(app, None);
                }
                "newprofile" => {
                    _open_settings_window(app, Some("/settings/newprofile".to_string()));
                }
                "open" => OpenAddFileDialogEvent.emit(app).unwrap(),
                "clear" => ClearFilesEvent.emit(app).unwrap(),
                "paste" => {
                    save_clipboard_image(app);
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
                        if res.is_err() {
                            println!("Error checking for updates: {:?}", res);
                            UpdateStateEvent::Error {
                                message: format!("{}", res.unwrap_err()),
                            }
                            .emit(&handle)
                            .unwrap();
                        }
                    });
                }

                _ => {}
            };
        })
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            let handle = app.handle().clone();
            // let clipboard = handle.state::<tauri_plugin_clipboard::Clipboard>();
            // match clipboard.start_monitor(handle.clone()) {
            //     Ok(_) => {
            //         println!("Clipboard monitor started");
            //     }
            //     Err(e) => {
            //         println!("Error starting clipboard monitor: {:?}", e);
            //     }
            // }
            // This is also required for events
            builder.mount_events(app);
            app.listen("pasteImage", move |_event| {
                save_clipboard_image(&handle);
            });

            // Store setup
            app.store("settings.json")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
