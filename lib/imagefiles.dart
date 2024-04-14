import 'dart:async';
import 'dart:io';

import 'package:alic/compressor.dart';
import 'package:flutter/material.dart';
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
    if (result.endsWith("0" * round)) {
      result = result.substring(0, result.length - round - 1);
    }

    return "$result ${affixes[affix]}";
  }
}

class ImageFormats {
  static const jpeg = ['jpeg', 'jpg'];
  static const png = ['png'];
  static const gif = ['gif'];
  static const webp = ['webp'];
  static const all = [...jpeg, ...png, ...gif, ...webp];
  static contains(String ext) {
    return all.contains(ext.toLowerCase());
  }

  static isImage(String path) {
    final ext = path.split('.').last;
    return all.contains(ext.toLowerCase());
  }
}

enum ImageFileStatus {
  pending('Pending'),
  compressing('Compressing'),
  success('Success'),
  unoptimized('Unoptimized'),
  error('Error');

  static const doneStatuses = [
    ImageFileStatus.success,
    ImageFileStatus.unoptimized,
    ImageFileStatus.error
  ];

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
    return switch (status) {
      ImageFileStatus.success => sizeAfterOptimization! >= size
          ? '0%'
          : '${((1 - (sizeAfterOptimization!.toDouble() / size)) * 100).toStringAsFixed(2)}%',
      ImageFileStatus.error => '-',
      ImageFileStatus.unoptimized => '-',
      _ => '?',
    };
  }

  String get file {
    return path.split('/').last;
  }

  String get sizeHumanReadable {
    return tableSize.toHumanReadableFileSize();
  }

  int get tableSize {
    return sizeAfterOptimization ?? size;
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
  static final _updateQueue = <ImageFile>[];
  static Timer? _debounce;

  static void add(List<File> files) {
    var images = <ImageFile>[];
    for (var file in files) {
      final imageFile = _add(file);
      if (imageFile != null) {
        images.add(imageFile);
      }
    }
    for (var file in images) {
      update(file);
      compress(file);
    }
  }

  static ImageFile? _add(File file) {
    if (!ImageFormats.isImage(file.path)) {
      return null;
    }
    if (signal.any((element) => element.path == file.path)) {
      return null;
    }
    final imageFile = ImageFile(
      path: file.path,
      size: file.lengthSync(),
      sizeAfterOptimization: null,
      status: ImageFileStatus.pending,
    );
    return imageFile;
  }

  static void compress(ImageFile file) {
    compressor(file, (p0) {
      debugPrint('Update: $p0');
      update(p0);
    });
  }

  static void update(ImageFile file) {
    _updateQueue.add(file);
    if (_debounce?.isActive ?? false) return;
    _debounce = Timer(const Duration(milliseconds: 200), () {
      var files = [..._updateQueue];
      _updateQueue.clear();
      var oldFiles = signal.value.toList();
      for (var file in files) {
        final index =
            oldFiles.indexWhere((element) => element.path == file.path);
        if (index == -1) {
          oldFiles.add(file);
        } else {
          oldFiles[index] = file;
        }
      }
      signal.value = oldFiles;
    });
  }

  static String dataSaved() {
    final saved = signal
        .where((element) => element.sizeAfterOptimization != null)
        .map((e) => e.size - e.sizeAfterOptimization!)
        .fold<int>(0, (previousValue, element) => previousValue + element);
    return saved.toHumanReadableFileSize();
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

  static isProcessing() {
    return signal
        .any((element) => element.status == ImageFileStatus.compressing);
  }

  static void removeDone() {
    signal.removeWhere(
        (element) => ImageFileStatus.doneStatuses.contains(element.status));
  }
}
