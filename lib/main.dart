import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';
import 'package:imageoptimflutter/config.dart';
import 'package:imageoptimflutter/imagefiles.dart';
import 'package:imageoptimflutter/src/rust/frb_generated.dart';
import 'package:signals/signals_flutter.dart';
import 'package:window_manager/window_manager.dart';

import 'table.dart';

class Settings {
  int jpegQuality;
  int pngQuality;
  int gifQuality;
  Settings(
      {required this.jpegQuality,
      required this.pngQuality,
      required this.gifQuality});
}

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await windowManager.ensureInitialized();
  Config.init();

  WindowOptions windowOptions = const WindowOptions(
    minimumSize: Size(600, 400),
    size: Size(600, 400),
    center: true,
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
      title: 'Flutter Demo',
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
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return const Scaffold(
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Expanded(
            child: FilesTable(),
          ),
          BottomBar(),
        ],
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
              constraints: const BoxConstraints.tightFor(width: 37, height: 37),
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
                message = '${files.length} files added';
              }
              return Text(message,
                  style: const TextStyle(color: Colors.white70, fontSize: 12));
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
                    // bottom sheet with settings

                    showDialog(
                        context: context,
                        builder: (context) {
                          return const SettingsWidget();
                        });
                  },
                  icon: const Icon(Icons.settings),
                  iconSize: 20,
                  color: Colors.white70,
                  padding: const EdgeInsets.all(0),
                ),
                TextButton.icon(
                  onPressed: () {
                    ImageFiles.signal.value = [];
                  },
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white70,
                  ),
                  icon: const Icon(Icons.clear),
                  label: const Text('Clear'),
                ),
              ],
            ),
          ],
        ));
  }
}

class SettingsWidget extends StatelessWidget {
  const SettingsWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return SimpleDialog(
      backgroundColor: const Color(0xFF2D2D2D),
      title: const Text('Settings'),
      children: [
        Padding(
          padding: const EdgeInsets.all(10),
          child: Watch(
            (context) {
              final config = Config.signal.value;
              return Column(
                children: [
                  Row(
                    children: [
                      const Text('JPEG quality'),
                      const Spacer(),
                      SliderWidget(
                          value: config.qualityJPEG,
                          onChanged: (value) {
                            Config.signal.value = Config.signal.value
                                .copyWith(qualityJPEG: value);
                          }),
                    ],
                  ),
                  Row(
                    children: [
                      const Text('PNG quality'),
                      const Spacer(),
                      SliderWidget(
                          value: config.qualityPNG,
                          onChanged: (value) {
                            Config.signal.value =
                                Config.signal.value.copyWith(qualityPNG: value);
                          }),
                    ],
                  ),
                  Row(
                    children: [
                      const Text('GIF quality'),
                      const Spacer(),
                      SliderWidget(
                          value: config.qualityGIF,
                          onChanged: (value) {
                            Config.signal.value =
                                Config.signal.value.copyWith(qualityGIF: value);
                          }),
                    ],
                  ),
                  Row(
                    children: [
                      const Text('WEBP quality'),
                      const Spacer(),
                      SliderWidget(
                          value: config.qualityWEBP,
                          onChanged: (value) {
                            Config.signal.value = Config.signal.value
                                .copyWith(qualityWEBP: value);
                          }),
                    ],
                  ),
                ],
              );
            },
          ),
        ),
        Row(
          children: [
            const Spacer(),
            TextButton(
                onPressed: () {
                  Config.reset();
                },
                child: const Text('Reset')),
            TextButton(
                onPressed: () {
                  Navigator.pop(context);
                },
                child: const Text('Close')),
            const Spacer(),
          ],
        )
      ],
    );
  }
}

class SliderWidget extends StatelessWidget {
  const SliderWidget({super.key, required this.value, required this.onChanged});
  final int value;
  final void Function(int) onChanged;
  @override
  Widget build(BuildContext context) {
    return Slider(
      value: value.toDouble(),
      max: 100,
      min: 10,
      divisions: 90,
      label: value.round().toString(),
      onChanged: (double value) {
        onChanged(value.round());
      },
    );
  }
}
