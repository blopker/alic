import 'dart:io';

import 'package:alic/filesystem.dart';
import 'package:alic/log.dart';
import 'package:flutter/services.dart';

registerDockDropHandler(Function(List<File>) onDrop) async {
  const platform = MethodChannel('io.kbl.alic');
  platform.setMethodCallHandler((call) async {
    log.d(call);
    if (call.method == 'openImage') {
      var path = call.arguments as String;
      var paths = await resolvePaths([path]);
      onDrop(paths);
    }
  });
}
