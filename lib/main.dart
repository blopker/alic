import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

import './table.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Must add this line.
  await windowManager.ensureInitialized();

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
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a purple toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.

        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple)
            .copyWith(background: const Color(0xFF1B1B1B)),
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
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      // This call to setState tells the Flutter framework that something has
      // changed in this State, which causes it to rerun the build method below
      // so that the display can reflect the updated values. If we changed
      // _counter without calling setState(), then the build method would not be
      // called again, and so nothing would appear to happen.
      _counter++;
    });
  }

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
            child: FilesTable(rows: [
              {
                'status': 0,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
              {
                'status': 1,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
              {
                'status': 0,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
              {
                'status': 1,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
              {
                'status': 3,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
              {
                'status': 1,
                'file': 'bo.jpg',
                'size': '123,123',
                'savings': '12,%'
              },
            ]),
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
                FilePickerResult? result =
                    await FilePicker.platform.pickFiles(allowMultiple: true);

                if (result != null) {
                  List<File> files =
                      result.paths.map((path) => File(path!)).toList();
                  print(files);
                } else {
                  // User canceled the picker
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
            const Expanded(
                child: Text(
                    'Lossy minification enabled (JPEG 85%, PNG 80%, GIF 80%)',
                    style: TextStyle(color: Colors.white70, fontSize: 12))),
            const SizedBox(
              width: 10,
            ),
            Row(
              children: [
                IconButton(
                  constraints:
                      const BoxConstraints.tightFor(width: 37, height: 37),
                  onPressed: () {},
                  icon: const Icon(Icons.settings),
                  iconSize: 20,
                  color: Colors.white70,
                  padding: const EdgeInsets.all(0),
                ),
                TextButton.icon(
                  onPressed: () {},
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white70,
                  ),
                  icon: const Icon(Icons.refresh),
                  label: const Text('Again'),
                ),
              ],
            ),
          ],
        ));
  }
}
