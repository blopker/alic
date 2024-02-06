import 'package:flutter/material.dart';
import 'package:imageoptimflutter/config.dart';
import 'package:signals/signals_flutter.dart';

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
