import 'dart:async';
import 'dart:io';

import 'package:imageoptimflutter/imagefiles.dart';
import 'package:imageoptimflutter/src/rust/api/simple.dart';

Future<ImageFile> _compress(ImageFile imageFile) async {
  // Compress the image file by adding it to the queue, then run the callback when done.
  final ext = imageFile.path.split('.').last;
  final outPath = imageFile.path.replaceAll('.$ext', '_compressed2.$ext');
  var result = await imgcompress(path: imageFile.path, outPath: outPath);
  print('Compressed ${imageFile.path} to $outPath with result $result');
  return imageFile.copyWith(
    sizeAfterOptimization: File(outPath).lengthSync(),
    status: result,
  );
}

void compressor(ImageFile imageFile, void Function(ImageFile) callback) {
  _compress(imageFile).then(callback);
}
