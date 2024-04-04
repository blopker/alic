import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:signals/signals.dart' as signals;

part 'config.freezed.dart';
part 'config.g.dart';

// A config file is created to store the configuration of the application.
// Stored in $HOME/.config/alic/config.json

Timer? _debounce;

@freezed
class ConfigData with _$ConfigData {
  const factory ConfigData({
    @Default(80) int qualityJPEG,
    @Default(true) bool lossy,
    @Default(80) int qualityPNG,
    @Default(60) int qualityWEBP,
    @Default(80) int qualityGIF,
    @Default(false) bool resizeImages,
    @Default(1920) int maxWidth,
    @Default(1080) int maxHeight,
    @Default(true) bool enablePostfix,
    @Default('.min') String postfix,
    @Default(ThemeMode.system) ThemeMode themeMode,
  }) = _ConfigData;

  factory ConfigData.fromJson(Map<String, Object?> json) =>
      _$ConfigDataFromJson(json);
}

final configDir = Directory('${Platform.environment['HOME']}/.config/alic');
final configFile = File('${configDir.path}/config.json');

class Config {
  static final signals.Signal<ConfigData> signal =
      signals.signal(const ConfigData());

  static void init() {
    signal.value = readConfig();
    signals.effect(() {
      debugPrint('Config changed: ${signal.value}');
      writeConfig(signal.value);
    });
  }

  static ConfigData readConfig() {
    ensureConfigExists();
    var configData = configFile.readAsStringSync();
    debugPrint('Read config: $configData');
    return ConfigData.fromJson(jsonDecode(configData));
  }

  static void writeConfig(ConfigData configData) {
    ensureConfigExists();
    if (_debounce?.isActive ?? false) _debounce?.cancel();
    _debounce = Timer(const Duration(milliseconds: 500), () {
      configFile.writeAsStringSync(jsonEncode(configData.toJson()));
    });
  }

  static void ensureConfigExists() {
    if (!configDir.existsSync()) {
      configDir.createSync(recursive: true);
    }
    if (!configFile.existsSync() || configFile.lengthSync() == 0) {
      configFile.writeAsStringSync(jsonEncode(signal.value.toJson()));
    }
  }

  static void reset() {
    var config = const ConfigData();
    writeConfig(config);
    signal.value = config;
  }
}
