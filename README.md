<a name="readme-top"></a>

<br />
<div align="center">
  <a href="https://github.com/blopker/alic">
    <img src="app-icon.png" alt="Logo" width="100" >
  </a>
  <h3 align="center">Alic</h3>
  <p align="center">
    Alic ('Al-ik') is a little image compressor for macOS.
    <br />
    <br />
    <a href="https://github.com/blopker/alic/releases/latest/">Download</a>
    <br />
    <br />
    <a href="https://github.com/blopker/alic/issues">Report Bug</a>
    ·
    <a href="https://github.com/blopker/alic/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About

[![Product Name Screen Shot][product-screenshot]](alic2-sc.min.png)

Alic makes it simple to compress images. It's a great tool for quickly compressing images for the web, or to share with clients.

Why compress images?

- Faster load times
- Less bandwidth usage
- Better SEO
- Better user experience

And security: Many cameras and phones embed metadata in images, which can include location, camera model, and other sensitive information. Alic can remove this metadata for you.

Supported image formats:

- JPEG
- PNG
- WebP
- GIF
- TIFF

Alic is heavily inspired by [ImageOptim][imageoptim-url], but with modern compression algorithms for smaller files and speed. See [Differences from ImageOptim](#differences).

## Installation

1. Download the latest release from the [releases page][project-release-url]
2. Drag to Applications folder
3. Launch and enjoy!

## Usage

Drag and drop images (or folders) into the window, images will automatically start compressing. The compressed images will be saved in the same directory as the original images.

Having trouble? Hover over the status icons for helpful hints!

### CLI Usage

The Alic app binary doubles as a CLI. Pass `--input` to compress images without opening the GUI:

```
Alic --input /path/to/image.jpg
Alic --input /path/to/images/ --reformat webp --lossy --webp-quality 80
```

On macOS you can run the binary inside the app bundle:

```
"/Applications/Alic Image Compressor.app/Contents/MacOS/Alic" --input /path/to/image.jpg
```

Or create a shell alias for convenience:

```
alias alic='"/Applications/Alic Image Compressor.app/Contents/MacOS/Alic"'
```

Run `Alic --help` for all options. Boolean flags use `--flag` / `--no-flag` pairs (e.g. `--lossy` / `--no-lossy`). Omitted flags use the selected profile's value.

Flags:

- `--input <path>` — Input file or directory (required, repeatable)
- `--profile <name-or-id>` — Profile to use
- `--threads <n>` — Concurrent image processing (default: 1)
- `--recursive` / `--no-recursive` — Recurse into directories (default: recursive)
- `--resize <WIDTHxHEIGHT>` — Resize images
- `--reformat <format>` — Convert (jpeg|png|webp|gif|tiff|avif)
- `--overwrite` / `--no-overwrite`
- `--postfix <text>` — Postfix text for output filenames
- `--add-postfix` / `--no-postfix`
- `--lossy` / `--no-lossy`
- `--keep-metadata` / `--no-keep-metadata`
- `--keep-timestamps` / `--no-keep-timestamps`
- `--background-fill <#RRGGBB>` / `--no-background-fill`
- `--jpeg-quality <1-100>`
- `--png-quality <1-100>`
- `--webp-quality <1-100>`
- `--gif-quality <1-100>`
- `--avif-quality <1-100>`
- `--help` / `--version`

## Privacy

Your data stays on your machine! No sneaky analytics or tracking here. Alic doesn't phone home, which means you'll need to manually check for updates through the [releases page][project-release-url] or by clicking "Check for Updates" in the app menu bar.

## <a name="differences"></a>Differences from ImageOptim

- WebP support
- Super fast: Built with Rust and modern compression magic
- Safe defaults: Original files stay untouched
- Resize option: Scale down those massive images
- Multiple profiles: Save different settings for different needs

## Roadmap

- Apple Developer ID signing
- Update checker
- Different optimization levels
- Lossless compression
- Directory support
- Finder context menu
- Image resizing

## Built With

Built with love using SolidJS and Rust, powered by Tauri. Image compression magic by [`libcaesium`][libcaesium-url]!

### Requirements

Runs on MacOS 12.0 or later

## Contributing

We love contributions! Want to make Alic better? Here's how:

1. Fork it
2. Create your feature branch
3. Make your changes
4. Push to the branch
5. Open a Pull Request

## Release

To release a new version of Alic:

1. Update `CHANGELOG.md` and commit.
2. Run `make release`.
3. When prompted, press Enter to accept the suggested patch version bump, or enter a custom version (e.g., `2.5.0` for minor, `3.0.0` for major).

The release script automatically updates the version in `tauri.conf.json` and `Cargo.toml`, commits the change, and pushes to the release branch.

## License

Distributed under the GNU 3.0 License. See `LICENSE` for more information.

## Contact

Use the issue tracker at the [Project Link][project-url].

## Acknowledgments

This project would not be possible without the following open source projects:

- For compression: [libcaesium][libcaesium-url]
- Original inspiration: [ImageOptim][imageoptim-url]
- UI: [SolidJS][solidjs-url]
- Application framework: [Tauri][tauri-url]

[license-url]: https://github.com/blopker/alic/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/blopker
[product-screenshot]: alic2-sc.min.png
[libcaesium-url]: https://github.com/Lymphatus/libcaesium
[tauri-url]: https://tauri.app/
[imageoptim-url]: https://imageoptim.com/mac
[project-url]: https://github.com/blopker/alic
[project-release-url]: https://github.com/blopker/alic/releases
[solidjs-url]: https://www.solidjs.com/
