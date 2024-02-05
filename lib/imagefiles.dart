import 'dart:io';

import 'package:flutter/material.dart';
import 'package:imageoptimflutter/compressor.dart';
import 'package:signals/signals.dart' as signals;

extension FileSizeExtensions on num {
  /// method returns a human readable string representing a file size
  /// size can be passed as number or as string
  /// the optional parameter 'round' specifies the number of numbers after comma/point (default is 2)
  /// the optional boolean parameter 'useBase1024' specifies if we should count in 1024's (true) or 1000's (false). e.g. 1KB = 1024B (default is true)
  String toHumanReadableFileSize({int round = 2, bool useBase1024 = true}) {
    const List<String> affixes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];

    num divider = useBase1024 ? 1024 : 1000;

    num size = this;
    num runningDivider = divider;
    num runningPreviousDivider = 0;
    int affix = 0;

    while (size >= runningDivider && affix < affixes.length - 1) {
      runningPreviousDivider = runningDivider;
      runningDivider *= divider;
      affix++;
    }

    String result =
        (runningPreviousDivider == 0 ? size : size / runningPreviousDivider)
            .toStringAsFixed(round);

    //Check if the result ends with .00000 (depending on how many decimals) and remove it if found.
    if (result.endsWith("0" * round))
      result = result.substring(0, result.length - round - 1);

    return "$result ${affixes[affix]}";
  }
}

enum ImageFileStatus {
  pending('Pending'),
  compressing('Compressing'),
  success('Success'),
  unoptimized('Unoptimized'),
  error('Error');

  final String value;

  const ImageFileStatus(this.value);
}

@immutable
class ImageFile {
  final String path;
  final int size;
  final int? sizeAfterOptimization;
  final ImageFileStatus status;
  final String? errorMessage;

  String get savings {
    if (sizeAfterOptimization == null) {
      return '?';
    }
    return '${((1 - (sizeAfterOptimization!.toDouble() / size)) * 100).toStringAsFixed(2)}%';
  }

  String get file {
    return path.split('/').last;
  }

  String get sizeHumanReadable {
    return size.toHumanReadableFileSize();
  }

  const ImageFile({
    required this.path,
    required this.size,
    required this.sizeAfterOptimization,
    required this.status,
    this.errorMessage,
  });

  ImageFile copyWith({
    String? name,
    String? path,
    int? size,
    int? sizeAfterOptimization,
    ImageFileStatus? status,
    String? errorMessage,
  }) {
    errorMessage = status == ImageFileStatus.error ? errorMessage : null;
    return ImageFile(
        path: path ?? this.path,
        size: size ?? this.size,
        sizeAfterOptimization:
            sizeAfterOptimization ?? this.sizeAfterOptimization,
        status: status ?? this.status,
        errorMessage: errorMessage);
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
      status: ImageFileStatus.pending,
    );
    if (signal.any((element) => element.path == file.path)) {
      return;
    }
    signal.add(file);
    compress(file);
  }

  static void compress(ImageFile file) {
    final newFile = file.copyWith(status: ImageFileStatus.compressing);
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

  static String dataSaved() {
    final saved = signal
        .where((element) => element.sizeAfterOptimization != null)
        .map((e) => e.size - e.sizeAfterOptimization!)
        .fold<int>(0, (previousValue, element) => previousValue + element);
    return '$saved bytes';
  }

  static String averagePercentSaved() {
    final saved = signal
        .where((element) => element.sizeAfterOptimization != null)
        .map((e) => 1 - (e.sizeAfterOptimization! / e.size))
        .fold<double>(0, (previousValue, element) => previousValue + element);
    return '${(saved / signal.length * 100).toStringAsFixed(2)}%';
  }

  static int totalFiles() {
    return signal.length;
  }

  static int totalFilesOptimized() {
    return signal
        .where((element) => element.sizeAfterOptimization != null)
        .length;
  }

  static int totalFilesPending() {
    return signal
        .where((element) => element.sizeAfterOptimization == null)
        .length;
  }
}
