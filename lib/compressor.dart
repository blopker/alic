import 'dart:io';

import 'package:alic/imagefiles.dart';
import 'package:alic/src/rust/api/compressor.dart';
import 'package:alic/workqueue.dart';
import 'package:flutter/foundation.dart';

import './config.dart';

const compressionThreshold = 0.95;

void compressor(ImageFile imageFile, void Function(ImageFile) callback) {
  // Compress the image file by adding it to the queue, then run the callback when done.
  final config = Config.signal.value;
  final ext = imageFile.path.split('.').last;
  final outPath = imageFile.path.replaceAll('.$ext', '${config.postfix}.$ext');

  workQueue.add(() async {
    callback(imageFile.copyWith(status: ImageFileStatus.compressing));
    final timer = Stopwatch()..start();

    var params = Parameters(
      jpegQuality: config.qualityJPEG,
      pngQuality: config.qualityPNG,
      gifQuality: config.qualityGIF,
      webpQuality: config.qualityWEBP,
      resize: config.resizeImages,
      resizeWidth: config.maxWidth,
      resizeHeight: config.maxHeight,
    );

    var result = await processImg(
      path: imageFile.path,
      outPath: outPath,
      parameters: params,
    );
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
    final outFile = File(outPath);
    final sizeAfterOptimization = await outFile.length();
    if (sizeAfterOptimization.toDouble() / imageFile.size >
        compressionThreshold) {
      // delete the file if it's not smaller
      outFile.delete();
      callback(imageFile.copyWith(
        status: ImageFileStatus.unoptimized,
      ));
      return;
    }
    // Success!
    if (!config.enablePostfix) {
      // If postfix is disabled, replace the original file with the optimized one
      File(imageFile.path).delete();
      outFile.rename(imageFile.path);
    }
    callback(imageFile.copyWith(
      sizeAfterOptimization: sizeAfterOptimization,
      status: ImageFileStatus.success,
    ));
  });
}
