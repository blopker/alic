use std::mem;
extern crate libc;
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
        println!("Result: {:?}", String::from_utf8(output.stdout));
    } else {
        println!("Exit with code: {}", output.status.code().unwrap());
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
            "hw.ncpu\0".as_ptr() as *const i8,
            &mut num_cores as *mut _ as *mut libc::c_void,
            &mut len,
            core::ptr::null_mut(),
            0,
        );
        num_cores
    }
}

pub fn trash_file(file_path: &str) -> Result<(), String> {
    unsafe {
        let url = NSURL::fileURLWithPath(&NSString::from_str(file_path));
        let result =
            NSFileManager::defaultManager().trashItemAtURL_resultingItemURL_error(&url, None);

        if result.is_err() {
            let err = result.err().unwrap();
            // println!("Failed to move file to Trash: {:?}", err);
            return Err(err.to_string());
        }
    }
    Ok(())
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
