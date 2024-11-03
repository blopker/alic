import 'dart:io';

import 'package:flutter/services.dart';

import 'log.dart';

class NativeFileManager {
  static const MethodChannel _channel = MethodChannel('io.kbl.alic');

  static Future<void> trashItem(String filePath) async {
    try {
      await _channel.invokeMethod('trash', {'filePath': filePath});
    } on PlatformException catch (e) {
      log.d("Failed to trash item: '${e.message}'.");
    }
  }
}

trash(File file) {
  NativeFileManager.trashItem(file.path);
}
