import 'package:alic/dropper.dart';
import 'package:alic/imagefiles.dart';
import 'package:alic/settings.dart';
import 'package:alic/src/rust/frb_generated.dart';
import 'package:alic/table.dart';
import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';
import 'package:signals/signals_flutter.dart';
import 'package:window_manager/window_manager.dart';

import './config.dart';
import 'glass.dart';
import 'theme.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await windowManager.ensureInitialized();
  Config.init();

  WindowOptions windowOptions = const WindowOptions(
    minimumSize: Size(600, 400),
    skipTaskbar: false,
  );
  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
    await windowManager.focus();
  });
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final config = Config.signal;
    return MaterialApp(
      title: 'Alic',
      theme: const MaterialTheme(TextTheme()).light(),
      darkTheme: const MaterialTheme(TextTheme()).dark(),
      home: const HomePage(),
      themeMode: config.watch(context).themeMode,
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: MyDropRegion(
        onDrop: (path) {
          debugPrint('Dropped: $path');
          ImageFiles.add(path);
        },
        dropOverlay: Container(
            color: Colors.transparent,
            child: const Center(
              child: Icon(
                Icons.file_download,
                size: 400,
              ),
            )).asGlass(
          tintColor: Colors.transparent,
        ),
        child: const Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: FilesTable(),
            ),
            BottomBar(),
          ],
        ),
      ),
    );
  }
}

class BottomBar extends StatelessWidget {
  const BottomBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
        height: 50,
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.secondaryContainer,
        ),
        child: Row(
          children: [
            const SizedBox(
              width: 5,
            ),
            // button with plus icon, and a thin border with radius around it
            IconButton(
              onPressed: () async {
                const XTypeGroup jpgsTypeGroup = XTypeGroup(
                  label: 'JPEGs',
                  extensions: <String>['jpg', 'jpeg'],
                );
                const XTypeGroup pngTypeGroup = XTypeGroup(
                  label: 'PNGs',
                  extensions: <String>['png'],
                );
                const XTypeGroup gifsTypeGroup = XTypeGroup(
                  label: 'GIFs',
                  extensions: <String>['gif'],
                );
                const XTypeGroup webpTypeGroup = XTypeGroup(
                  label: 'WEBPs',
                  extensions: <String>['webp'],
                );
                final List<XFile> files = await openFiles(
                    acceptedTypeGroups: <XTypeGroup>[
                      jpgsTypeGroup,
                      pngTypeGroup,
                      gifsTypeGroup,
                      webpTypeGroup
                    ]);
                for (var file in files) {
                  ImageFiles.add(file.path);
                }
              },
              icon: const Icon(Icons.add),
              iconSize: 20,
              padding: const EdgeInsets.all(0),
            ),
            const SizedBox(
              width: 10,
            ),
            Expanded(child: Watch((context) {
              var files = ImageFiles.signal;
              var config = Config.signal;
              var message = 'No files added';
              if (!config().enablePostfix) {
                message += ', overwriting original files';
              }
              if (config().resizeImages) {
                message += ', resizing images';
              }
              if (files.isNotEmpty) {
                final filePlural = files.length == 1 ? 'file' : 'files';
                message =
                    '${ImageFiles.dataSaved()} saved over ${files.length} $filePlural, average ${ImageFiles.averagePercentSaved()}';
              }
              if (ImageFiles.isProcessing()) {
                message += ' Processing...';
              }
              return Text(message, style: const TextStyle(fontSize: 12));
            })),
            const SizedBox(
              width: 10,
            ),
            Row(
              children: [
                IconButton(
                  constraints:
                      const BoxConstraints.tightFor(width: 37, height: 37),
                  onPressed: () {
                    showDialog(
                        context: context,
                        builder: (context) {
                          return const SettingsWidget();
                        });
                  },
                  icon: const Icon(Icons.settings),
                  iconSize: 20,
                  padding: const EdgeInsets.all(0),
                ),
                Watch(
                  (_) => TextButton.icon(
                    onPressed: ImageFiles.signal.isEmpty
                        ? null
                        : () {
                            ImageFiles.removeDone();
                          },
                    icon: const Icon(
                      Icons.close,
                      size: 20,
                    ),
                    label: const Text('Clear'),
                  ),
                ),
              ],
            ),
          ],
        ));
  }
}
