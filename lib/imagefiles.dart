import 'dart:io';

import 'package:flutter/material.dart';
import 'package:imageoptimflutter/compressor.dart';
import 'package:signals/signals.dart' as signals;

@immutable
class ImageFile {
  final String path;
  final int size;
  final int? sizeAfterOptimization;
  final String status;

  String get savings {
    if (sizeAfterOptimization == null) {
      return '?';
    }
    return '${((1 - (sizeAfterOptimization!.toDouble() / size)) * 100).toStringAsFixed(2)}%';
  }

  String get file {
    return path.split('/').last;
  }

  const ImageFile({
    required this.path,
    required this.size,
    required this.sizeAfterOptimization,
    required this.status,
  });

  ImageFile copyWith({
    String? name,
    String? path,
    int? size,
    int? sizeAfterOptimization,
    String? status,
  }) {
    return ImageFile(
      path: path ?? this.path,
      size: size ?? this.size,
      sizeAfterOptimization:
          sizeAfterOptimization ?? this.sizeAfterOptimization,
      status: status ?? this.status,
    );
  }

  Map<String, dynamic> toJson() => {
        'path': path,
        'file': file,
        'size': size,
        'sizeAfterOptimization': sizeAfterOptimization,
        'status': status,
      };

  @override
  String toString() {
    return toJson().toString();
  }
}

class ImageFiles {
  static final signal = signals.listSignal<ImageFile>([]);

  static void add(String path) {
    final file0 = File(path);
    final file = ImageFile(
      path: file0.path,
      size: file0.lengthSync(),
      sizeAfterOptimization: null,
      status: 'Pending',
    );
    if (signal.any((element) => element.path == file.path)) {
      return;
    }
    signal.add(file);
    compress(file);
  }

  static void compress(ImageFile file) {
    final newFile = file.copyWith(status: 'Compressing');
    update(newFile);
    compressor(newFile, (p0) {
      print('Compressed: $p0');
      update(p0);
    });
  }

  static void update(ImageFile file) {
    final index = signal.indexWhere((element) => element.path == file.path);
    print(index);
    if (index == -1) {
      return;
    }
    signal[index] = file;
  }
}
