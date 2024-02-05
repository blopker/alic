import 'dart:io';

import 'package:imageoptimflutter/imagefiles.dart';
import 'package:imageoptimflutter/src/rust/api/simple.dart';
import 'package:imageoptimflutter/workqueue.dart';

void compressor(ImageFile imageFile, void Function(ImageFile) callback) {
  // Compress the image file by adding it to the queue, then run the callback when done.
  final ext = imageFile.path.split('.').last;
  final outPath = imageFile.path.replaceAll('.$ext', '.min.$ext');

  workQueue.add(() async {
    var result = await imgcompress(path: imageFile.path, outPath: outPath);
    if (result.toLowerCase().contains('error')) {
      callback(imageFile.copyWith(
        status: ImageFileStatus.error,
        errorMessage: result,
      ));
      return;
    }
    callback(imageFile.copyWith(
      sizeAfterOptimization: File(outPath).lengthSync(),
      status: ImageFileStatus.success,
    ));
  });
}
