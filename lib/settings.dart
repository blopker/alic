import 'package:alic/config.dart';
import 'package:flutter/material.dart';
import 'package:signals/signals_flutter.dart';

enum SettingsPages {
  general('General'),
  quality('Quality');
  // advanced('Advanced');

  const SettingsPages(this.title);

  final String title;
}

class SettingsWidget extends StatefulWidget {
  const SettingsWidget({super.key});

  @override
  State<SettingsWidget> createState() => _SettingsWidgetState();
}

class _SettingsWidgetState extends State<SettingsWidget> {
  SettingsPages _selectedPage = SettingsPages.general;

  @override
  Widget build(BuildContext context) {
    return SimpleDialog(
      shadowColor: Colors.black,
      backgroundColor: const Color(0xFF2D2D2D),
      children: [
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 5),
          child: SegmentedButton(
            selectedIcon: Container(),
            style: ButtonStyle(
              foregroundColor: MaterialStateProperty.resolveWith((states) {
                if (states.contains(MaterialState.selected)) {
                  return Colors.black;
                }
                return Colors.white70;
              }),
            ),
            onSelectionChanged: (p0) {
              final newPage = SettingsPages.values.firstWhere(
                  (element) => element.toString() == p0.first.toString());
              setState(() {
                _selectedPage = newPage;
              });
            },
            segments: SettingsPages.values
                .map((e) =>
                    ButtonSegment(value: e.toString(), label: Text(e.title)))
                .toList(),
            selected: <dynamic>{_selectedPage.toString()},
          ),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 10),
          child: _getSelectedPage(),
        ),
        Row(
          children: [
            const Spacer(),
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

  Widget _getSelectedPage() {
    switch (_selectedPage) {
      case SettingsPages.general:
        return const GeneralPage();
      case SettingsPages.quality:
        return const QualityPage();
      // case SettingsPages.advanced:
      //   return const AdvancedPage();
    }
  }
}

class QualitySliderWidget extends StatelessWidget {
  const QualitySliderWidget(
      {super.key, required this.value, required this.onChanged});
  final int value;
  final void Function(int) onChanged;
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Slider(
        value: value.toDouble(),
        max: 100,
        min: 10,
        divisions: 10,
        label: value.round().toString(),
        onChanged: (double value) {
          onChanged(value.round());
        },
      ),
    );
  }
}

class SliderWidget extends StatelessWidget {
  const SliderWidget({super.key, required this.value, required this.onChanged});
  final int value;
  final void Function(int) onChanged;
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Slider(
        value: value.toDouble(),
        max: 100,
        min: 0,
        divisions: 10,
        label: value.round().toString(),
        onChanged: (double value) {
          onChanged(value.round());
        },
      ),
    );
  }
}

class GeneralPage extends StatelessWidget {
  const GeneralPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Watch(
      (context) {
        final config = Config.signal.value;
        return Column(
          children: [
            Row(
              children: [
                const Text('Overwrite original files'),
                const Spacer(),
                Switch(
                  value: !config.enablePostfix,
                  onChanged: (value) {
                    Config.signal.value =
                        Config.signal.value.copyWith(enablePostfix: !value);
                  },
                ),
              ],
            ),
            // Row(
            //   children: [
            //     const Text('Postfix'),
            //     const Spacer(),
            //     SizedBox(
            //       width: 100,
            //       child: TextField(
            //         controller: TextEditingController(
            //             text: postfixValue == '.min' ? '' : postfixValue),
            //         onTapOutside: (value) {
            //           final newVal = value.isEmpty ? '.min' : value;
            //           Config.signal.value =
            //               Config.signal.value.copyWith(postfix: newVal);
            //         },
            //         decoration: const InputDecoration(
            //           labelText: '.min',
            //         ),
            //       ),
            //     )
            //   ],
            // ),
          ],
        );
      },
    );
  }
}

class QualityPage extends StatelessWidget {
  const QualityPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Watch(
      (context) {
        final config = Config.signal.value;
        return Column(
          children: [
            Row(
              children: [
                const Text('JPEG quality'),
                const Spacer(),
                QualitySliderWidget(
                    value: config.qualityJPEG,
                    onChanged: (value) {
                      Config.signal.value = config.copyWith(qualityJPEG: value);
                    }),
              ],
            ),
            Row(
              children: [
                const Text('PNG quality'),
                const Spacer(),
                QualitySliderWidget(
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
                QualitySliderWidget(
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
                QualitySliderWidget(
                    value: config.qualityWEBP,
                    onChanged: (value) {
                      Config.signal.value =
                          Config.signal.value.copyWith(qualityWEBP: value);
                    }),
              ],
            ),
            const SizedBox(height: 10),
            TextButton(
                onPressed: () {
                  Config.reset();
                },
                child: const Text('Reset')),
          ],
        );
      },
    );
  }
}

class AdvancedPage extends StatelessWidget {
  const AdvancedPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const Text('Advanced');
  }
}
