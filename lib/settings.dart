import 'package:alic/config.dart';
import 'package:alic/tooltip.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:signals/signals_flutter.dart';
import 'package:custom_sliding_segmented_control/custom_sliding_segmented_control.dart';
import 'src/rust/api/compressor.dart';

enum SettingsPages {
  general('General'),
  quality('Quality'),
  resize('Resize');
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
    return Dialog(
      child: KeyboardListener(
        focusNode: FocusNode(),
        onKeyEvent: (event) {
          var nextPage = SettingsPages.values[
              (SettingsPages.values.indexOf(_selectedPage) + 1) %
                  SettingsPages.values.length];
          if (event.logicalKey == LogicalKeyboardKey.tab) {
            setState(() {
              _selectedPage = nextPage;
            });
          }
        },
        child: Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(children: [
            CustomSlidingSegmentedControl<SettingsPages>(
              initialValue: _selectedPage,
              clipBehavior: Clip.antiAlias,
              children: SettingsPages.values.fold({}, (map, element) {
                map[element] = Text(element.title,
                    style: TextStyle(
                        color: Theme.of(context).colorScheme.primary));
                return map;
              }),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.secondaryContainer,
                borderRadius: BorderRadius.circular(16),
              ),
              thumbDecoration: BoxDecoration(
                color: Theme.of(context)
                    .colorScheme
                    .primaryContainer
                    .withOpacity(0.6),
                borderRadius: BorderRadius.circular(16),
              ),
              customSegmentSettings: CustomSegmentSettings(
                borderRadius: BorderRadius.circular(25),
              ),
              onValueChanged: (newPage) {
                setState(() {
                  _selectedPage = newPage;
                });
              },
            ),
            const SizedBox(height: 10),
            _getSelectedPage(),
            const Spacer(),
            Row(
              children: [
                TextButton(
                    onPressed: () {
                      showDialog(
                        context: context,
                        builder: (BuildContext context) {
                          return AlertDialog(
                            title: const Text('Reset Settings'),
                            content: const Text(
                                'Are you sure you want to reset all settings to the defaults?'),
                            actions: <Widget>[
                              TextButton(
                                child: const Text('Cancel'),
                                onPressed: () {
                                  Navigator.of(context).pop();
                                },
                              ),
                              TextButton(
                                child: const Text('Reset'),
                                onPressed: () {
                                  Config.reset();
                                  Navigator.of(context).pop();
                                },
                              ),
                            ],
                          );
                        },
                      );
                    },
                    child: const Text('Reset')),
                const Spacer(),
                TextButton(
                    onPressed: () {
                      Navigator.pop(context);
                    },
                    child: const Text('Close')),
              ],
            )
          ]),
        ),
      ),
    );
  }

  Widget _getSelectedPage() {
    switch (_selectedPage) {
      case SettingsPages.general:
        return const GeneralPage();
      case SettingsPages.quality:
        return const QualityPage();
      case SettingsPages.resize:
        return const ResizePage();
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

class GeneralPage extends StatelessWidget {
  const GeneralPage({super.key});

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    return Watch(
      (context) {
        final config = Config.signal.value;
        return Column(
          children: [
            Row(
              children: [
                const Text('Theme'),
                const Spacer(),
                DropdownButton<ThemeMode>(
                    items: const [
                      DropdownMenuItem(
                        value: ThemeMode.system,
                        child: Text('System'),
                      ),
                      DropdownMenuItem(
                        value: ThemeMode.light,
                        child: Text('Light'),
                      ),
                      DropdownMenuItem(
                        value: ThemeMode.dark,
                        child: Text('Dark'),
                      ),
                    ],
                    value: config.themeMode,
                    onChanged: (v) {
                      if (v == null) return;
                      Config.signal.value = config.copyWith(themeMode: v);
                    }),
              ],
            ),
            Row(
              children: [
                const Text('Overwrite original files'),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: AlicTooltip(
                    message:
                        'Original files will be overwritten with the compressed version. Original files are moved to the Trash. '
                        'If disabled, the compressed version will be saved in the same directory as the original file, with a postfix.',
                    child: Icon(Icons.help, color: theme.hintColor),
                  ),
                ),
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
            Row(
              children: [
                const Text('Convert format to'),
                const Spacer(),
                DropdownButton<ImageType?>(
                    items: [
                      DropdownMenuItem(value: null, child: Text('Original')),
                      ...ImageType.values.map((e) => DropdownMenuItem(
                            value: e,
                            child: Text(e.name),
                          )),
                    ],
                    value: config.convertExtension,
                    onChanged: (v) {
                      Config.signal.value =
                          config.copyWith(convertExtension: v);
                    }),
              ],
            ),
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
          ],
        );
      },
    );
  }
}

class ResizePage extends StatelessWidget {
  const ResizePage({super.key});

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    return Watch(
      (context) {
        final config = Config.signal.value;
        return Column(
          children: [
            const SizedBox(height: 10),
            Text(
              'Resize images to fit within the specified dimensions. Keeps original aspect ratio. Does not enlarge images.',
              style: theme.textTheme.labelSmall,
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                const Text('Resize images'),
                const Spacer(),
                Switch(
                  value: config.resizeImages,
                  onChanged: (value) {
                    Config.signal.value =
                        Config.signal.value.copyWith(resizeImages: value);
                  },
                ),
              ],
            ),
            Row(
              children: [
                const Text('Maximum width'),
                const Spacer(),
                SizedBox(
                    width: 150,
                    child: TextFormField(
                        initialValue: config.maxWidth.toString(),
                        keyboardType: TextInputType.number,
                        inputFormatters: [
                          FilteringTextInputFormatter.digitsOnly
                        ],
                        decoration:
                            const InputDecoration(labelText: 'Max width (px)'),
                        onChanged: (value) {
                          if (value.isEmpty) return;
                          Config.signal.value = Config.signal.value
                              .copyWith(maxWidth: int.parse(value));
                        })),
              ],
            ),
            Row(
              children: [
                const Text('Maximum height'),
                const Spacer(),
                SizedBox(
                    width: 150,
                    child: TextFormField(
                        initialValue: config.maxWidth.toString(),
                        keyboardType: TextInputType.number,
                        inputFormatters: [
                          FilteringTextInputFormatter.digitsOnly
                        ],
                        decoration:
                            const InputDecoration(labelText: 'Max height (px)'),
                        onChanged: (value) {
                          if (value.isEmpty) return;
                          Config.signal.value = Config.signal.value
                              .copyWith(maxHeight: int.parse(value));
                        })),
              ],
            ),
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
