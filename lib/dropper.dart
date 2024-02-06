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
  double opacity = 0;

  final formats = const [
    Formats.jpeg,
    Formats.png,
    Formats.gif,
    Formats.webp,
    folderFormat
  ];
  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return SizedBox(
          width: constraints.maxWidth,
          height: constraints.maxHeight,
          child: DropRegion(
            formats: formats,
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
              setState(() {
                opacity = 1;
              });
            },
            onDropLeave: (event) {
              setState(() {
                opacity = 0;
              });
            },
            onPerformDrop: (event) async {
              debugPrint('onPerformDrop');
              final items = event.session.items;
              for (var item in items) {
                for (var format in formats) {
                  if (item.canProvide(format)) {
                    final reader = item.dataReader!;
                    reader.getValue(Formats.fileUri, (value) {
                      widget.onDrop(value!.toFilePath());
                    });
                  }
                }
              }
            },
            child: Stack(children: [
              widget.child,
              Visibility(
                  maintainAnimation: true,
                  maintainState: true,
                  visible: opacity != 0,
                  child: AnimatedOpacity(
                      opacity: opacity,
                      duration: const Duration(milliseconds: 100),
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
