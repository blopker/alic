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

  workQueue.add(() async {
    callback(imageFile.copyWith(status: ImageFileStatus.compressing));
    final timer = Stopwatch()..start();

    var params = Parameters(
        postfix: config.postfix,
        path: imageFile.path,
        jpegQuality: config.qualityJPEG,
        pngQuality: config.qualityPNG,
        gifQuality: config.qualityGIF,
        webpQuality: config.qualityWEBP,
        resize: config.resizeImages,
        resizeWidth: config.maxWidth,
        resizeHeight: config.maxHeight,
        convertExtension: config.convertExtension);

    CompressResult result;
    try {
      result = await processImg(
        parameters: params,
      );
    } catch (e) {
      callback(imageFile.copyWith(
        status: ImageFileStatus.error,
        errorMessage: e.toString(),
      ));
      return;
    } finally {
      timer.stop();
    }

    debugPrint(
        'Compressed ${imageFile.file} in ${timer.elapsedMilliseconds}ms');

    final outFile = File(result.outPath);
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
