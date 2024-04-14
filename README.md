<a name="readme-top"></a>

<br />
<div align="center">
  <a href="https://github.com/blopker/alic">
    <img src="assets/alic-logo.png" alt="Logo" width="100" >
  </a>
  <h3 align="center">Alic</h3>
  <p align="center">
    Alic ('Alice') is A Little Image Compressor for MacOS.
    <br />
    <br />
    <a href="https://github.com/blopker/alic/releases/latest/download/Alic.Image.Compressor.dmg">Download</a>
    <br />
    <br />
    <a href="https://github.com/blopker/alic/issues">Report Bug</a>
    ·
    <a href="https://github.com/blopker/alic/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

[![Product Name Screen Shot][product-screenshot]](alic-sc.min.png)

Alic is a image compressor for MacOS. It has simple drag and drop interface that allows you to lossy compress images. It's a great tool for quickly compressing images for the web.

Why compress images?

- Faster load times
- Less bandwidth usage
- Better SEO
- Better user experience

Alic also removes metadata from images, which is a privacy concern. Many cameras and phones embed metadata in images, which can include location, camera model, and other sensitive information.

Supported image formats:

- JPEG
- PNG
- WebP
- GIF

Alic is heavily inspired by [ImageOptim][imageoptim-url], but with modern compression algorithms for smaller files and speed.

## Installation

Start by downloading the latest release from the [releases page][project-release-url]. Then, drag the app to your Applications folder and open it.

**The first time you open Alic**, you will need to hold down the `option` key and right click the app in the Applications folder. Then click "Open" and then "Open" again. This is because Alic is not signed with an Apple Developer ID yet.

## Usage

Drag and drop images into the window, images will automatically start compressing. The compressed images will be saved in the same directory as the original images.

**Careful**, compressing already compressed images times will result in a loss of quality.

## Privacy

All compression is done locally on your machine. Alic also does not have any analytics or tracking, including error reporting. Alic does not send any data to the internet. Because of this, Alic will not automatically update. You will need to check the [releases page][project-release-url] for updates.

## Differences from ImageOptim

- Alic's compressor is written in Rust, uses modern compression algorithms and is done all in process. This makes Alic noticeably faster, and produces smaller file sizes.
- Alic does not overwrite the original images by default, it saves the compressed images in the same directory as the original images with a `.min.` suffix. Alic can be configured to overwrite the original images.
- Alic can resize images if they are over a certain size. Images will not be resized by default, and images will not be upscaled.

## Roadmap

- [ ] Get the app signed with an Apple Developer ID
- [x] Add support for different optimization levels
- [ ] Add support for lossless compression
- [x] Add support for dropping directories
- [ ] Add context menu for opening images in Alic from Finder
- [x] Add support for resizing images if they are over a certain size

## Built With

Alic a GUI for the [`libcaesium`][libcaesium-url] Rust library, with a UI written in Flutter.

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

1. Update the version in `pubspec.yaml`.
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
- Rust interop: [flutter_rust_bridge][flutter-rust-bridge-url]
- UI: [Flutter][flutter-url]

[license-url]: https://github.com/blopker/alic/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/blopker
[product-screenshot]: alic-sc.min.png
[libcaesium-url]: https://github.com/Lymphatus/libcaesium
[flutter-rust-bridge-url]: https://cjycode.com/flutter_rust_bridge/
[flutter-url]: https://flutter.dev/
[imageoptim-url]: https://imageoptim.com/mac
[project-url]: https://github.com/blopker/alic
[project-release-url]: https://github.com/blopker/alic/releases
