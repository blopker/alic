<a name="readme-top"></a>

<br />
<div align="center">
  <a href="https://github.com/blopker/alic">
    <img src="assets/alic-logo.png" alt="Logo" width="100" >
  </a>
  <h3 align="center">Alic</h3>
  <p align="center">
    Alic ('Alice') is a simple image compressor for MacOS.
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

Alic is a simple image compressor for MacOS. It's a GUI for the `libcaesium` Rust library, with a UI written in Flutter. It has simple drag and drop interface that allows you to lossy compress images. It's a great tool for quickly compressing images for the web.

Supported image formats:

- JPEG
- PNG
- WebP
- GIF

Alic heavily inspired by [ImageOptim](https://imageoptim.com/mac), but with modern compression algorithms for better compression and speed.

## Usage

Start by downloading the latest release from the [releases page](https://github.com/blopker/alic/releases). Then, drag the app to your Applications folder and open it.

Drag and drop images into the window, images will automatically start compressing. The compressed images will be saved in the same directory as the original images.

**Careful**: compressing already compressed images times will result in a loss of quality.

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

## License

Distributed under the GNU 3.0 License. See `LICENSE` for more information.

## Contact

Project Link: [https://github.com/blopker/alic](https://github.com/blopker/alic)

## Acknowledgments ❤️

This project would not be possible without the following open source projects:

- For compression: [libcaesium](https://github.com/Lymphatus/libcaesium)
- Original inspirations: [ImageOptim](https://imageoptim.com/mac)
- Rust interop: [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/)
- UI: [Flutter](https://flutter.dev/)

[license-url]: https://github.com/blopker/alic/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/blopker
[product-screenshot]: alic-sc.min.png
