use std::mem;
extern crate libc;
use objc2_app_kit::{NSColor, NSColorSpace};
use objc2_foundation::{NSFileManager, NSString, NSURL};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
#[specta::specta]
pub async fn open_finder_at_path(path: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let output = app_handle
        .shell()
        .command("open")
        .args(["-R", path.as_str()])
        .output()
        .await
        .unwrap();
    if output.status.success() {
        let result = String::from_utf8(output.stdout);
        println!("Result: {result:?}");
    } else {
        let code = output.status.code().unwrap();
        println!("Exit with code: {code}");
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_cpu_count() -> i32 {
    unsafe {
        let mut num_cores = 0;
        let mut len = mem::size_of::<libc::size_t>() as libc::size_t;
        libc::sysctlbyname(
            c"hw.ncpu".as_ptr(),
            &mut num_cores as *mut _ as *mut libc::c_void,
            &mut len,
            core::ptr::null_mut(),
            0,
        );
        num_cores
    }
}

pub fn trash_file(file_path: &str) -> Result<(), String> {
    let url = NSURL::fileURLWithPath(&NSString::from_str(file_path));
    let result = NSFileManager::defaultManager().trashItemAtURL_resultingItemURL_error(&url, None);

    if result.is_err() {
        let err = result.err().unwrap();
        // println!("Failed to move file to Trash: {:?}", err);
        return Err(err.to_string());
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_accent_color() -> Result<[u8; 4], String> {
    let rgba;
    unsafe {
        let accent = NSColor::controlAccentColor();
        let color_space = NSColorSpace::genericRGBColorSpace();
        let accent = accent.colorUsingColorSpace(&color_space);
        rgba = accent.map(|accent| {
            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
            accent.getRed_green_blue_alpha(&mut r, &mut g, &mut b, &mut a);
            [
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                (a * 255.0) as u8,
            ]
        });
    }
    match rgba {
        Some(color) => Ok(color),
        None => Err("Failed to get color".to_string()),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_trash() {
    //     trash_file("/Users/blopker/Downloads/zhv025yxv(18).png".to_string());
    //     assert!(false);
    // }
}
