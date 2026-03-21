use crate::compress::{ImageType, gather_image_paths, process_path};
use crate::errors::AlicErrorType;
use crate::settings::{ProfileData, SettingsData};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

// Must match "identifier" in tauri.conf.json
const APP_IDENTIFIER: &str = "io.kbl.alic";

#[derive(Clone, Debug)]
pub struct CliConfig {
    pub inputs: Vec<String>,
    pub recursive: bool,
    pub threads: usize,
    pub profile: Option<String>,
    pub overrides: CliOverrides,
}

#[derive(Clone, Debug, Default)]
pub struct CliOverrides {
    pub should_overwrite: Option<bool>,
    pub should_convert: Option<bool>,
    pub convert_extension: Option<ImageType>,
    pub should_resize: Option<bool>,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,
    pub add_postfix: Option<bool>,
    pub postfix: Option<String>,
    pub enable_lossy: Option<bool>,
    pub keep_timestamps: Option<bool>,
    pub keep_metadata: Option<bool>,
    pub should_background_fill: Option<bool>,
    pub background_fill: Option<String>,
    pub jpeg_quality: Option<u32>,
    pub png_quality: Option<u32>,
    pub webp_quality: Option<u32>,
    pub gif_quality: Option<u32>,
    pub avif_quality: Option<u32>,
}

pub fn run() -> i32 {
    let args: Vec<String> = env::args().skip(1).collect();
    match run_with_args(&args) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

pub fn run_with_args(args: &[String]) -> Result<(), String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help") {
        print_help();
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config = parse_args(args)?;
    let settings = load_settings_data()?;
    let base_profile = resolve_profile(&settings, config.profile.as_deref())?;
    let profile = apply_overrides(base_profile, &config.overrides);

    let mut paths = Vec::new();
    for input in &config.inputs {
        let discovered = gather_image_paths(input, config.recursive);
        paths.extend(discovered);
    }
    dedupe_paths(&mut paths);
    if paths.is_empty() {
        return Err("No supported image files found in --input paths".to_string());
    }

    let thread_count = config.threads;
    let (tx, rx) = mpsc::channel();

    // Process images in chunks of `thread_count` for concurrency
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
            handle.join().unwrap();
        }
    }
    drop(tx);

    let mut ok = 0_u32;
    let mut already_smaller = 0_u32;
    let mut errors = 0_u32;
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

pub fn parse_args(args: &[String]) -> Result<CliConfig, String> {
    let mut cfg = CliConfig {
        inputs: Vec::new(),
        recursive: true,
        threads: 1,
        profile: None,
        overrides: CliOverrides::default(),
    };

    let mut index = 0_usize;
    while index < args.len() {
        let arg = &args[index];
        if !arg.starts_with("--") {
            return Err(format!(
                "Unexpected argument `{arg}`. Only long-form flags are supported."
            ));
        }
        let trimmed = &arg[2..];
        let (key, value, consumed_next) = split_flag(trimmed, args, index)?;
        index += if consumed_next { 2 } else { 1 };

        match key.as_str() {
            "input" => cfg.inputs.push(value),
            "recursive" => cfg.recursive = parse_bool(&value, &key)?,
            "profile" => cfg.profile = Some(value),
            "threads" => {
                let parsed = value
                    .parse::<usize>()
                    .map_err(|_| "Invalid --threads value".to_string())?;
                if parsed == 0 {
                    return Err("--threads must be greater than 0".to_string());
                }
                cfg.threads = parsed;
            }
            "resize" => {
                let (w, h) = parse_resize(&value)?;
                cfg.overrides.should_resize = Some(true);
                cfg.overrides.resize_width = Some(w);
                cfg.overrides.resize_height = Some(h);
            }
            "reformat" => {
                cfg.overrides.should_convert = Some(true);
                cfg.overrides.convert_extension = Some(parse_image_type(&value)?);
            }
            "overwrite" => cfg.overrides.should_overwrite = Some(parse_bool(&value, &key)?),
            "postfix" => cfg.overrides.postfix = Some(value),
            "add-postfix" => cfg.overrides.add_postfix = Some(parse_bool(&value, &key)?),
            "lossy" => cfg.overrides.enable_lossy = Some(parse_bool(&value, &key)?),
            "keep-timestamps" => cfg.overrides.keep_timestamps = Some(parse_bool(&value, &key)?),
            "keep-metadata" => cfg.overrides.keep_metadata = Some(parse_bool(&value, &key)?),
            "enable-background-fill" => {
                cfg.overrides.should_background_fill = Some(parse_bool(&value, &key)?)
            }
            "background-fill" => {
                validate_hex_color(&value)?;
                cfg.overrides.background_fill = Some(value);
                if cfg.overrides.should_background_fill.is_none() {
                    cfg.overrides.should_background_fill = Some(true);
                }
            }
            "jpeg-quality" => cfg.overrides.jpeg_quality = Some(parse_quality(&value, &key)?),
            "png-quality" => cfg.overrides.png_quality = Some(parse_quality(&value, &key)?),
            "webp-quality" => cfg.overrides.webp_quality = Some(parse_quality(&value, &key)?),
            "gif-quality" => cfg.overrides.gif_quality = Some(parse_quality(&value, &key)?),
            "avif-quality" => cfg.overrides.avif_quality = Some(parse_quality(&value, &key)?),
            unknown => return Err(format!("Unknown flag --{unknown}")),
        }
    }

    if cfg.inputs.is_empty() {
        return Err("At least one --input=<path> is required".to_string());
    }
    Ok(cfg)
}

fn split_flag(
    trimmed: &str,
    args: &[String],
    index: usize,
) -> Result<(String, String, bool), String> {
    if let Some((key, value)) = trimmed.split_once('=') {
        if key.is_empty() || value.is_empty() {
            return Err(format!("Invalid flag format: --{trimmed}"));
        }
        return Ok((key.to_string(), value.to_string(), false));
    }
    let key = trimmed;
    if key.is_empty() {
        return Err("Invalid empty flag".to_string());
    }
    let next = args
        .get(index + 1)
        .ok_or_else(|| format!("Missing value for --{key}"))?;
    if next.starts_with("--") {
        return Err(format!("Missing value for --{key}"));
    }
    Ok((key.to_string(), next.clone(), true))
}

fn parse_bool(value: &str, key: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("--{key} expects true or false")),
    }
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

fn parse_quality(value: &str, key: &str) -> Result<u32, String> {
    let parsed = value
        .parse::<u32>()
        .map_err(|_| format!("--{key} expects a number between 1 and 100"))?;
    if !(1..=100).contains(&parsed) {
        return Err(format!("--{key} expects a number between 1 and 100"));
    }
    Ok(parsed)
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

fn apply_overrides(mut profile: ProfileData, overrides: &CliOverrides) -> ProfileData {
    if let Some(value) = overrides.should_overwrite {
        profile.should_overwrite = value;
    }
    if let Some(value) = overrides.should_convert {
        profile.should_convert = value;
    }
    if let Some(value) = overrides.convert_extension.clone() {
        profile.convert_extension = value;
    }
    if let Some(value) = overrides.should_resize {
        profile.should_resize = value;
    }
    if let Some(value) = overrides.resize_width {
        profile.resize_width = value;
    }
    if let Some(value) = overrides.resize_height {
        profile.resize_height = value;
    }
    if let Some(value) = overrides.add_postfix {
        profile.add_postfix = value;
    }
    if let Some(value) = overrides.postfix.clone() {
        profile.postfix = value;
    }
    if let Some(value) = overrides.enable_lossy {
        profile.enable_lossy = value;
    }
    if let Some(value) = overrides.keep_timestamps {
        profile.keep_timestamps = value;
    }
    if let Some(value) = overrides.keep_metadata {
        profile.keep_metadata = value;
    }
    if let Some(value) = overrides.should_background_fill {
        profile.should_background_fill = value;
    }
    if let Some(value) = overrides.background_fill.clone() {
        profile.background_fill = value;
    }
    if let Some(value) = overrides.jpeg_quality {
        profile.jpeg_quality = value;
    }
    if let Some(value) = overrides.png_quality {
        profile.png_quality = value;
    }
    if let Some(value) = overrides.webp_quality {
        profile.webp_quality = value;
    }
    if let Some(value) = overrides.gif_quality {
        profile.gif_quality = value;
    }
    if let Some(value) = overrides.avif_quality {
        profile.avif_quality = value;
    }
    profile
}

fn load_settings_data() -> Result<SettingsData, String> {
    let path = default_settings_path()
        .ok_or_else(|| "Unable to resolve settings directory from environment".to_string())?;
    if !path.exists() {
        return Ok(SettingsData::new());
    }
    let text = fs::read_to_string(&path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|err| format!("Invalid settings JSON: {err}"))?;

    // tauri-plugin-store usually wraps values in an object map by key.
    if let Some(settings_value) = value.get("settings") {
        return serde_json::from_value(settings_value.clone())
            .map_err(|err| format!("Invalid settings payload: {err}"));
    }
    serde_json::from_value(value).map_err(|err| format!("Invalid settings payload: {err}"))
}

fn default_settings_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = env::var("HOME").ok()?;
        return Some(
            Path::new(&home)
                .join("Library")
                .join("Application Support")
                .join(APP_IDENTIFIER)
                .join("settings.json"),
        );
    }
    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").ok()?;
        return Some(
            Path::new(&home)
                .join(".local")
                .join("share")
                .join(APP_IDENTIFIER)
                .join("settings.json"),
        );
    }
    #[cfg(target_os = "windows")]
    {
        let app_data = env::var("APPDATA").ok()?;
        return Some(
            Path::new(&app_data)
                .join(APP_IDENTIFIER)
                .join("settings.json"),
        );
    }
    #[allow(unreachable_code)]
    None
}

fn print_help() {
    println!("alic-cli --input=<path> [--input=<path> ...] [options]");
    println!("Options:");
    println!("  --profile=<name-or-id>");
    println!("  --threads=<n>  (number of images to process concurrently)");
    println!("  --recursive=<true|false>");
    println!("  --resize=<WIDTHxHEIGHT>");
    println!("  --reformat=<jpeg|png|webp|gif|tiff|avif>");
    println!("  --overwrite=<true|false>");
    println!("  --postfix=<text>");
    println!("  --add-postfix=<true|false>");
    println!("  --jpeg-quality=<1-100>");
    println!("  --png-quality=<1-100>");
    println!("  --webp-quality=<1-100>");
    println!("  --gif-quality=<1-100>");
    println!("  --avif-quality=<1-100>");
    println!("  --lossy=<true|false>");
    println!("  --keep-metadata=<true|false>");
    println!("  --keep-timestamps=<true|false>");
    println!("  --enable-background-fill=<true|false>");
    println!("  --background-fill=<#RRGGBB>");
    println!("  --help");
    println!("  --version");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resize_and_reformat_flags() {
        let args = vec![
            "--input=fixtures".to_string(),
            "--resize=123x456".to_string(),
            "--reformat=jpg".to_string(),
            "--threads=3".to_string(),
        ];
        let parsed = parse_args(&args).expect("args should parse");
        assert_eq!(parsed.inputs, vec!["fixtures".to_string()]);
        assert_eq!(parsed.threads, 3);
        assert_eq!(parsed.overrides.resize_width, Some(123));
        assert_eq!(parsed.overrides.resize_height, Some(456));
        assert_eq!(parsed.overrides.convert_extension, Some(ImageType::JPEG));
        assert_eq!(parsed.overrides.should_convert, Some(true));
    }

    #[test]
    fn rejects_short_flags() {
        let args = vec!["-i".to_string(), "fixtures".to_string()];
        let err = parse_args(&args).expect_err("short flag must fail");
        assert!(err.contains("Only long-form flags"));
    }

    #[test]
    fn validates_quality_range() {
        let args = vec![
            "--input=a.png".to_string(),
            "--jpeg-quality=101".to_string(),
        ];
        let err = parse_args(&args).expect_err("quality must fail");
        assert!(err.contains("between 1 and 100"));
    }
}
