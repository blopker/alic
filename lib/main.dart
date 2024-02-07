import 'package:alic/dropper.dart';
import 'package:alic/imagefiles.dart';
import 'package:alic/src/rust/frb_generated.dart';
import 'package:alic/table.dart';
import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';
import 'package:signals/signals_flutter.dart';
import 'package:window_manager/window_manager.dart';

import 'glass.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await windowManager.ensureInitialized();
  // Config.init();

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
    return MaterialApp(
      title: 'Alic',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple)
            .copyWith(background: const Color(0xFF1B1B1B)),
        textTheme: const TextTheme(
          bodyLarge: TextStyle(color: Colors.white),
          bodyMedium: TextStyle(color: Colors.white),
          bodySmall: TextStyle(color: Colors.white),
          headlineLarge: TextStyle(color: Colors.white),
          headlineMedium: TextStyle(color: Colors.white),
          headlineSmall: TextStyle(color: Colors.white),
          titleLarge: TextStyle(color: Colors.white),
          titleMedium: TextStyle(color: Colors.white),
          titleSmall: TextStyle(color: Colors.white),
        ),
      ),
      home: const HomePage(),
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
                Icons.download_sharp,
                color: Colors.white70,
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
        decoration: const BoxDecoration(
          //#2D2D2D
          color: Color(0xFF2D2D2D),
          border: Border(
            top: BorderSide(
              color: Colors.black,
              width: 0.5,
            ),
          ),
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
              color: Colors.white70,
              padding: const EdgeInsets.all(0),
            ),
            const SizedBox(
              width: 10,
            ),
            Expanded(child: Watch((context) {
              var files = ImageFiles.signal;
              var message = 'No files added';
              if (files.isNotEmpty) {
                final filePlural = files.length == 1 ? 'file' : 'files';
                message =
                    '${ImageFiles.dataSaved()} saved over ${files.length} $filePlural, average ${ImageFiles.averagePercentSaved()}';
              }
              if (ImageFiles.isProcessing()) {
                message += ' Processing...';
              }
              return Text(message,
                  style: const TextStyle(color: Colors.white70, fontSize: 12));
            })),
            const SizedBox(
              width: 10,
            ),
            Row(
              children: [
                // IconButton(
                //   constraints:
                //       const BoxConstraints.tightFor(width: 37, height: 37),
                //   onPressed: () {
                //     showDialog(
                //         context: context,
                //         builder: (context) {
                //           return const SettingsWidget();
                //         });
                //   },
                //   icon: const Icon(Icons.settings),
                //   iconSize: 20,
                //   color: Colors.white70,
                //   padding: const EdgeInsets.all(0),
                // ),
                Watch(
                  (_) => TextButton.icon(
                    onPressed: ImageFiles.signal.isEmpty
                        ? null
                        : () {
                            ImageFiles.removeDone();
                          },
                    style: TextButton.styleFrom(
                      foregroundColor: Colors.white70,
                    ),
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
