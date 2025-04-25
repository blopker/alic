# Unreleased

# 2.1.3

- Update deps

# 2.1.2

- Update deps
- Clippy fixes

# 2.1.1

- Fix spelling error in config file
- Refresh settings sidebar look

# 2.1.0

- Use the disk less when processing. This is a major speedup when compressing many images.
- Update to Rust 2024.

# 2.0.12

- Fix #9: 'Open with...' on file now works when Alic is closed.

# 2.0.11

- Update a few dependencies
- Set strict CSP header

# 2.0.10

- Fix white flicker when windows are opening

# 2.0.9

- Alic will now detect file formats by reading the file's signature. If the file's original extension is incorrect, Alic will output the right file extension.

# 2.0.8

- Fix white scroll bars when show scroll bars is set to always

# 2.0.7

- Try to get updater working

# 2.0.6

- Allow for upper case extensions

# 2.0.5

- Add update checker

# 2.0.4

- App is now signed by Apple
- Added metadata and lossless options

# 2.0.1

- Fix release script

# 2.0.0

- Complete rewrite to use Tauri and SolidJS
- Add profiles
- Add paste images from clipboard
- Add configurable parallelism
- New keyboard shortcuts

# 1.5.0

- You can now drop images and folders into the app icon on the dock
- Added ability to convert image formats in settings
- New settings page selector
- Overwritten images will now be moved to the Trash, instead of deleted
- Added a fun animation when compressing images
- New help text for some settings
- Performance and memory improvements
- Update dependencies

# 1.4.0

- Animated WebP support
- Fix small memory leak
- Update dependencies

# 1.3.3

- Update to the newest version of Rust

# 1.3.2

- Migrate to new Flutter color names
- Update various dependencies, crush images even faster now

# 1.3.1

- Update various dependencies

# 1.3.0

- Added a way to check for updates. Click "Check for Updates" in the app menu bar to see if there is a new version available.
- Update some dependencies

# 1.2.2

- Fixed a bug where the app would crash on first startup. Thanks @jsardev!

# 1.2.1

- Added a more helpful status message on startup showing activated settings.

# 1.2.0

- Added support for resizing images. New tab in the settings page to set the maximum width and height of images.
- Add theme support, default to system theme
- Performance improvements by batching table updates
- Style tweaks

# 1.1.3

- Updated `libcaesium`
- Add changelog
