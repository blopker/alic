use crate::compress::{ImageType, gather_image_paths, process_path};
use crate::errors::AlicErrorType;
use crate::settings::{self, ProfileData, SettingsData};
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use tauri_plugin_cli::Matches;

/// Check whether the CLI plugin received any `--input` arguments.
/// Returns `true` if CLI processing was triggered (caller should exit).
pub fn handle_matches(app: &tauri::AppHandle, matches: Matches) -> bool {
    // If no --input was provided, this is a normal GUI launch.
    let inputs = match get_strings(&matches, "input") {
        Some(v) if !v.is_empty() => v,
        _ => return false,
    };

    let exit_code = match run_cli(app, &matches, inputs) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    };
    std::process::exit(exit_code);
}

fn run_cli(
    app: &tauri::AppHandle,
    matches: &Matches,
    inputs: Vec<String>,
) -> Result<(), String> {
    let settings = settings::get_settings_data(app)?.0;
    let profile_selector = get_string(matches, "profile");
    let base_profile = resolve_profile(&settings, profile_selector.as_deref())?;
    let profile = apply_overrides(base_profile, matches)?;

    let thread_count = match get_string(matches, "threads") {
        Some(v) => {
            let parsed = v
                .parse::<usize>()
                .map_err(|_| "Invalid --threads value".to_string())?;
            if parsed == 0 {
                return Err("--threads must be greater than 0".to_string());
            }
            parsed
        }
        None => 1,
    };

    let recursive = get_flag_pair(matches, "recursive", "no-recursive").unwrap_or(true);

    let mut paths = Vec::new();
    for input in &inputs {
        let discovered = gather_image_paths(input, recursive);
        paths.extend(discovered);
    }
    dedupe_paths(&mut paths);
    if paths.is_empty() {
        return Err("No supported image files found in --input paths".to_string());
    }

    let (tx, rx) = mpsc::channel();

    let mut panicked = 0_u32;
    for chunk in paths.chunks(thread_count) {
        let handles: Vec<_> = chunk
            .iter()
            .map(|path| {
                let tx = tx.clone();
                let profile = profile.clone();
                let path = path.clone();
                let parallel = thread_count as i32;
                thread::spawn(move || {
                    let result = process_path(profile, path.clone(), parallel);
                    tx.send((path, result)).unwrap();
                })
            })
            .collect();
        for handle in handles {
            if handle.join().is_err() {
                eprintln!("err\t<unknown>\tworker thread panicked");
                panicked += 1;
            }
        }
    }
    drop(tx);

    let mut ok = 0_u32;
    let mut already_smaller = 0_u32;
    let mut errors = panicked;
    for (path, result) in rx {
        match result {
            Ok(result) => {
                ok += 1;
                println!("ok\t{}\t{}", result.path, result.out_path);
            }
            Err(err) => match err.error_type {
                AlicErrorType::NotSmaller => {
                    already_smaller += 1;
                    println!("skip\t{}\t{}", path, err.error);
                }
                _ => {
                    errors += 1;
                    eprintln!("err\t{}\t{}", path, err.error);
                }
            },
        }
    }

    println!("summary\tok={ok}\tskipped={already_smaller}\terrors={errors}");
    if errors > 0 {
        return Err("One or more files failed".to_string());
    }
    Ok(())
}

fn dedupe_paths(paths: &mut Vec<String>) {
    let mut seen = HashSet::new();
    paths.retain(|p| seen.insert(p.clone()));
}

// --- Arg helpers ---

fn get_string(matches: &Matches, name: &str) -> Option<String> {
    matches
        .args
        .get(name)
        .and_then(|arg| arg.value.as_str().map(|s| s.to_string()))
}

fn get_strings(matches: &Matches, name: &str) -> Option<Vec<String>> {
    let arg = matches.args.get(name)?;
    if let Some(arr) = arg.value.as_array() {
        let strings: Vec<String> = arr.iter().filter_map(|v| v.as_str().map(String::from)).collect();
        if strings.is_empty() {
            None
        } else {
            Some(strings)
        }
    } else if let Some(s) = arg.value.as_str() {
        if s.is_empty() {
            None
        } else {
            Some(vec![s.to_string()])
        }
    } else {
        None
    }
}

fn has_flag(matches: &Matches, name: &str) -> bool {
    matches
        .args
        .get(name)
        .map(|a| a.occurrences > 0)
        .unwrap_or(false)
}

/// Check a --flag / --no-flag pair. Returns Some(true) if the positive flag
/// was passed, Some(false) if the negative flag was passed, None if neither.
fn get_flag_pair(matches: &Matches, name: &str, no_name: &str) -> Option<bool> {
    let pos = matches
        .args
        .get(name)
        .map(|a| a.occurrences > 0)
        .unwrap_or(false);
    let neg = matches
        .args
        .get(no_name)
        .map(|a| a.occurrences > 0)
        .unwrap_or(false);
    match (pos, neg) {
        (true, _) => Some(true),
        (_, true) => Some(false),
        _ => None,
    }
}

fn parse_quality(value: &str, key: &str) -> Result<u32, String> {
    let parsed = value
        .parse::<u32>()
        .map_err(|_| format!("--{key} expects a number between 1 and 100"))?;
    if !(1..=100).contains(&parsed) {
        return Err(format!("--{key} expects a number between 1 and 100"));
    }
    Ok(parsed)
}

fn parse_resize(value: &str) -> Result<(u32, u32), String> {
    let (w, h) = value
        .split_once('x')
        .ok_or_else(|| "Invalid --resize value. Use WIDTHxHEIGHT".to_string())?;
    let width = w
        .parse::<u32>()
        .map_err(|_| "Invalid width in --resize".to_string())?;
    let height = h
        .parse::<u32>()
        .map_err(|_| "Invalid height in --resize".to_string())?;
    if width == 0 || height == 0 {
        return Err("--resize dimensions must be greater than 0".to_string());
    }
    Ok((width, height))
}

fn parse_image_type(value: &str) -> Result<ImageType, String> {
    match value.to_ascii_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageType::JPEG),
        "png" => Ok(ImageType::PNG),
        "webp" => Ok(ImageType::WEBP),
        "gif" => Ok(ImageType::GIF),
        "tiff" | "tif" => Ok(ImageType::TIFF),
        "avif" => Ok(ImageType::AVIF),
        _ => Err("Unsupported --reformat value".to_string()),
    }
}

fn validate_hex_color(value: &str) -> Result<(), String> {
    if value.len() != 7 || !value.starts_with('#') {
        return Err("--background-fill expects #RRGGBB".to_string());
    }
    if !value.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
        return Err("--background-fill expects #RRGGBB".to_string());
    }
    Ok(())
}

// --- Profile resolution ---

fn resolve_profile(settings: &SettingsData, selector: Option<&str>) -> Result<ProfileData, String> {
    let selected = match selector {
        Some(value) => {
            if let Ok(id) = value.parse::<u32>() {
                settings.profiles.iter().find(|p| p.id == id).cloned()
            } else {
                settings.profiles.iter().find(|p| p.name == value).cloned()
            }
        }
        None => settings.profiles.iter().find(|p| p.active).cloned(),
    };
    selected
        .or_else(|| settings.profiles.first().cloned())
        .ok_or_else(|| "No profiles available in settings".to_string())
}

fn apply_overrides(mut profile: ProfileData, matches: &Matches) -> Result<ProfileData, String> {
    if let Some(v) = get_flag_pair(matches, "overwrite", "no-overwrite") {
        profile.should_overwrite = v;
    }
    if let Some(v) = get_string(matches, "reformat") {
        profile.should_convert = true;
        profile.convert_extension = parse_image_type(&v)?;
    }
    if let Some(v) = get_string(matches, "resize") {
        let (w, h) = parse_resize(&v)?;
        profile.should_resize = true;
        profile.resize_width = w;
        profile.resize_height = h;
    }
    if let Some(v) = get_flag_pair(matches, "add-postfix", "no-postfix") {
        profile.add_postfix = v;
    }
    if let Some(v) = get_string(matches, "postfix") {
        profile.postfix = v;
    }
    if let Some(v) = get_flag_pair(matches, "lossy", "no-lossy") {
        profile.enable_lossy = v;
    }
    if let Some(v) = get_flag_pair(matches, "keep-timestamps", "no-keep-timestamps") {
        profile.keep_timestamps = v;
    }
    if let Some(v) = get_flag_pair(matches, "keep-metadata", "no-keep-metadata") {
        profile.keep_metadata = v;
    }
    if let Some(v) = get_string(matches, "background-fill") {
        validate_hex_color(&v)?;
        profile.background_fill = v;
        profile.should_background_fill = true;
    } else if has_flag(matches, "no-background-fill") {
        profile.should_background_fill = false;
    }
    if let Some(v) = get_string(matches, "jpeg-quality") {
        profile.jpeg_quality = parse_quality(&v, "jpeg-quality")?;
    }
    if let Some(v) = get_string(matches, "png-quality") {
        profile.png_quality = parse_quality(&v, "png-quality")?;
    }
    if let Some(v) = get_string(matches, "webp-quality") {
        profile.webp_quality = parse_quality(&v, "webp-quality")?;
    }
    if let Some(v) = get_string(matches, "gif-quality") {
        profile.gif_quality = parse_quality(&v, "gif-quality")?;
    }
    if let Some(v) = get_string(matches, "avif-quality") {
        profile.avif_quality = parse_quality(&v, "avif-quality")?;
    }
    Ok(profile)
}

pub fn print_help() {
    println!("Alic Image Compressor v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: Alic --input <path> [--input <path> ...] [options]");
    println!();
    println!("Options:");
    println!("  --input <path>              Input file or directory (required, repeatable)");
    println!("  --profile <name-or-id>      Profile to use");
    println!("  --threads <n>               Images to process concurrently (default: 1)");
    println!("  --recursive / --no-recursive");
    println!("                              Recurse into directories (default: recursive)");
    println!("  --resize <WIDTHxHEIGHT>     Resize images");
    println!("  --reformat <format>          Convert (jpeg|png|webp|gif|tiff|avif)");
    println!("  --overwrite / --no-overwrite");
    println!("  --postfix <text>             Postfix text for output filenames");
    println!("  --add-postfix / --no-postfix");
    println!("  --lossy / --no-lossy");
    println!("  --keep-metadata / --no-keep-metadata");
    println!("  --keep-timestamps / --no-keep-timestamps");
    println!("  --background-fill <#RRGGBB> / --no-background-fill");
    println!("  --jpeg-quality <1-100>");
    println!("  --png-quality <1-100>");
    println!("  --webp-quality <1-100>");
    println!("  --gif-quality <1-100>");
    println!("  --avif-quality <1-100>");
    println!("  --help                       Show this help");
    println!("  --version                    Show version");
    println!();
    println!("Boolean flags override the selected profile. Omitted flags use the profile value.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_resize_valid() {
        let (w, h) = parse_resize("123x456").unwrap();
        assert_eq!(w, 123);
        assert_eq!(h, 456);
    }

    #[test]
    fn parse_resize_invalid() {
        assert!(parse_resize("abc").is_err());
        assert!(parse_resize("0x100").is_err());
    }

    #[test]
    fn parse_image_type_valid() {
        assert_eq!(parse_image_type("jpg").unwrap(), ImageType::JPEG);
        assert_eq!(parse_image_type("WEBP").unwrap(), ImageType::WEBP);
        assert_eq!(parse_image_type("avif").unwrap(), ImageType::AVIF);
    }

    #[test]
    fn parse_quality_valid() {
        assert_eq!(parse_quality("80", "test").unwrap(), 80);
        assert!(parse_quality("0", "test").is_err());
        assert!(parse_quality("101", "test").is_err());
    }
}
