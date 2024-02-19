import 'package:flutter/material.dart';

class ASlider extends StatelessWidget {
  const ASlider({super.key, required this.value, required this.onChanged});
  final double value;
  final void Function(double) onChanged;

  @override
  Widget build(BuildContext context) {
    return Slider(
      value: value,
      onChanged: onChanged,
    );
  }
}
