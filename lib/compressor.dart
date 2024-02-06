import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:imageoptimflutter/imagefiles.dart';
import 'package:imageoptimflutter/src/rust/api/simple.dart';
import 'package:imageoptimflutter/workqueue.dart';

void compressor(ImageFile imageFile, void Function(ImageFile) callback) {
  // Compress the image file by adding it to the queue, then run the callback when done.
  final ext = imageFile.path.split('.').last;
  final outPath = imageFile.path.replaceAll('.$ext', '.min.$ext');

  workQueue.add(() async {
    callback(imageFile.copyWith(status: ImageFileStatus.compressing));
    final timer = Stopwatch()..start();
    var result = await imgcompress(path: imageFile.path, outPath: outPath);
    timer.stop();
    debugPrint(
        'Compressed ${imageFile.file} in ${timer.elapsedMilliseconds}ms');
    if (result.toLowerCase().contains('error')) {
      callback(imageFile.copyWith(
        status: ImageFileStatus.error,
        errorMessage: result,
      ));
      return;
    }
    final sizeAfterOptimization = await File(outPath).length();
    if (sizeAfterOptimization.toDouble() / imageFile.size > 0.95) {
      // delete the file if it's not smaller
      File(outPath).delete();
      callback(imageFile.copyWith(
        status: ImageFileStatus.unoptimized,
      ));
      return;
    }
    callback(imageFile.copyWith(
      sizeAfterOptimization: sizeAfterOptimization,
      status: ImageFileStatus.success,
    ));
  });
}
