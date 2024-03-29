import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:super_clipboard/super_clipboard.dart';
import 'package:super_drag_and_drop/super_drag_and_drop.dart';

const folderFormat = SimpleFileFormat(
  uniformTypeIdentifiers: [
    'public.folder',
  ],
);

class MyDropRegion extends StatefulWidget {
  const MyDropRegion(
      {super.key,
      required this.child,
      required this.dropOverlay,
      required this.onDrop});
  final Widget child;
  final Widget dropOverlay;
  final Function(String) onDrop;

  @override
  State<MyDropRegion> createState() => _MyDropRegionState();
}

class _MyDropRegionState extends State<MyDropRegion> {
  bool _showOverlay = false;
  bool _overlayVisible = false;

  final formats = const [
    Formats.jpeg,
    Formats.png,
    Formats.gif,
    Formats.webp,
    folderFormat
  ];

  showOverlay() {
    setState(() {
      _overlayVisible = true;
    });
    setState(() {
      _showOverlay = true;
    });
  }

  hideOverlay() {
    setState(() {
      _showOverlay = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return SizedBox(
          width: constraints.maxWidth,
          height: constraints.maxHeight,
          child: DropRegion(
            formats: Formats.standardFormats,
            hitTestBehavior: HitTestBehavior.opaque,
            onDropOver: (event) {
              debugPrint('onDropOver');
              for (var item in event.session.items) {
                for (var format in formats) {
                  if (item.canProvide(format)) {
                    return DropOperation.copy;
                  }
                }
              }
              return DropOperation.none;
            },
            onDropEnter: (event) {
              debugPrint('onDropEnter');
              showOverlay();
            },
            onDropLeave: (event) {
              debugPrint('onDropLeave');
              hideOverlay();
            },
            onPerformDrop: (event) async {
              debugPrint('onPerformDrop');
              final items = event.session.items;
              final mixedPaths = <String>[];
              for (var item in items) {
                for (var format in formats) {
                  if (item.canProvide(format)) {
                    var uri = await _getValueFromItem(item);
                    if (uri != null) {
                      mixedPaths.add(uri);
                    }
                  }
                }
              }
              final paths = await _resolvePaths(mixedPaths);
              for (var path in paths) {
                widget.onDrop(path);
              }
            },
            child: Stack(children: [
              widget.child,
              Visibility(
                  maintainState: true,
                  visible: _overlayVisible,
                  child: AnimatedOpacity(
                      opacity: _showOverlay ? 1.0 : 0.0,
                      duration: const Duration(milliseconds: 200),
                      onEnd: () => setState(() {
                            _overlayVisible = _showOverlay;
                          }),
                      child: LayoutBuilder(
                        builder: (context, constraints) {
                          return SizedBox(
                            width: constraints.maxWidth,
                            height: constraints.maxHeight,
                            child: widget.dropOverlay,
                          );
                        },
                      ))),
            ]),
          ),
        );
      },
    );
  }
}

Future<String?> _getValueFromItem(DropItem item) async {
  final reader = item.dataReader!;
  final completer = Completer<String?>();
  reader.getValue(Formats.fileUri, (value) {
    if (value == null) {
      completer.complete(null);
    } else {
      completer.complete(value.toFilePath());
    }
  });
  return completer.future;
}

Future<List<String>> _resolvePaths(List<String> paths) async {
  List<String> resolvedPaths = [];
  for (var path in paths) {
    if (path.endsWith('/')) {
      resolvedPaths.addAll(await _getImagesFromDirectory(path)
          .then((imageFiles) => imageFiles.map((e) => e.path).toList()));
    } else {
      resolvedPaths.add(path);
    }
  }
  return resolvedPaths;
}

// Return a list of images from a directory, recursively
Future<List<File>> _getImagesFromDirectory(String path) async {
  var dir = Directory(path);

  List<File> imageFiles = [];

  await for (var entity in dir.list(recursive: true, followLinks: false)) {
    if (entity is File && _isImage(entity)) {
      imageFiles.add(entity);
    }
  }

  return imageFiles;
}

bool _isImage(File file) {
  var ext = file.path.split('.').last;
  return ['jpg', 'jpeg', 'png', 'gif', 'webp'].contains(ext.toLowerCase());
}
