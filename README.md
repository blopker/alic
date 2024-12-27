<a name="readme-top"></a>

<br />
<div align="center">
  <a href="https://github.com/blopker/alic">
    <img src="app-icon.png" alt="Logo" width="100" >
  </a>
  <h3 align="center">Alic</h3>
  <p align="center">
    Alic ('Al-ik') is A Little Image Compressor for MacOS.
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

## About The Project

[![Product Name Screen Shot][product-screenshot]](alic2-sc.min.png)

Alic makes it simple to compress images. It's a great tool for quickly compressing images for the web, or to share with clients.

Why compress images?

- Faster load times
- Less bandwidth usage
- Better SEO
- Better user experience

And security: Many cameras and phones embed metadata in images, which can include location, camera model, and other sensitive information. Alic can remove this metadata from your images.

Supported image formats:

- JPEG
- PNG
- WebP
- GIF
- TIFF

Alic is heavily inspired by [ImageOptim][imageoptim-url], but with modern compression algorithms for smaller files and speed.

## Installation

Start by downloading the latest release from the [releases page][project-release-url]. Then, drag the app to your Applications folder and open it.

## Usage

Drag and drop images into the window, images will automatically start compressing. The compressed images will be saved in the same directory as the original images.

**Careful**, compressing already compressed images times can result in a loss of quality.

## Privacy

All compression is done locally on your machine. Alic also does not have any analytics or tracking, including error reporting. Alic does not passively send any data to the internet. Because of this, Alic will not automatically update. You will need to check the [releases page][project-release-url] for updates, or by clicking "Check for Updates" in the app menu bar.

## Differences from ImageOptim

- Alic's compressor is written in Rust, uses modern compression algorithms and is done all in process. This makes Alic noticeably faster, and produces smaller file sizes.
- Alic does not overwrite the original images by default, it saves the compressed images in the same directory as the original images with a `.min.` suffix. Alic can be configured to overwrite the original images.
- Alic can resize images if they are over a certain size. Images will not be resized by default, and images will not be upscaled.
- Alic supports multiple profiles where you can save settings for any situation you might be in.

## Roadmap

- [x] Get the app signed with an Apple Developer ID
- [x] Add a way to check for updates
- [x] Add support for different optimization levels
- [x] Add support for lossless compression
- [x] Add support for dropping directories
- [x] Add context menu for opening images in Alic from Finder
- [x] Add support for resizing images if they are over a certain size

## Built With

The Alic UI is built with SolidJS, with all processing done in Rust, using Tauri. Image compression is handled by [`libcaesium`][libcaesium-url].

### Requirements

Alic will only run on MacOS 12.0 or later.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Release

To release a new version of Alic, follow these steps:

1. Update the version in `tauri.conf.json`.
1. Update `CHANGELOG.md`.
1. Commit the changes, but do not push.
1. Run `make release`.

## License

Distributed under the GNU 3.0 License. See `LICENSE` for more information.

## Contact

Use the issue tracker at the [Project Link][project-url].

## Acknowledgments ❤️

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
