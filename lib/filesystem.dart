import 'dart:io';

import 'package:alic/imagefiles.dart';

Future<List<File>> resolvePaths(List<String> paths) async {
  List<File> resolvedPaths = [];
  for (var path in paths) {
    final dir = Directory(path);
    final file = File(path);
    if (await dir.exists()) {
      resolvedPaths.addAll(await getImagesFromDirectory(dir));
    } else if (await file.exists()) {
      resolvedPaths.add(file);
    }
  }
  return resolvedPaths;
}

// Return a list of images from a directory, recursively
Future<List<File>> getImagesFromDirectory(Directory dir) async {
  return await dir
      .list(recursive: true, followLinks: false)
      .where((entity) => entity is File)
      .map((entity) => entity as File)
      .where((file) => ImageFormats.isImage(file.path))
      .toList();
}
