import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:signals/signals.dart' as signals;

// A config file is created to store the configuration of the application.
// Stored in $HOME/.config/alic/config.json

@immutable
class ConfigData {
  final bool lossy;
  final int qualityJPEG;
  final int qualityPNG;
  final int qualityWEBP;
  final int qualityGIF;
  final bool enablePostfix;
  final String postfix;

  const ConfigData({
    required this.lossy,
    required this.qualityJPEG,
    required this.qualityPNG,
    required this.qualityWEBP,
    required this.qualityGIF,
    required this.enablePostfix,
    required this.postfix,
  });

  static validateQuality(dynamic quality, int defaultValue) {
    if (quality is int) {
      return quality.clamp(10, 100);
    }
    return defaultValue;
  }

  ConfigData.fromJson(Map<String, dynamic> json)
      : lossy = json['lossy'] ?? true,
        qualityJPEG = validateQuality(json['qualityJPEG'], 80),
        qualityPNG = validateQuality(json['qualityPNG'], 80),
        qualityWEBP = validateQuality(json['qualityWEBP'], 60),
        qualityGIF = validateQuality(json['qualityGIF'], 80),
        enablePostfix = json['enablePostfix'] ?? true,
        postfix = json['postfix'] ?? '.min';

  Map<String, dynamic> toJson() => {
        'lossy': lossy,
        'qualityJPEG': qualityJPEG,
        'qualityPNG': qualityPNG,
        'qualityWEBP': qualityWEBP,
        'qualityGIF': qualityGIF,
        'enablePostfix': enablePostfix,
        'postfix': postfix,
      };

  ConfigData copyWith({
    bool? lossy,
    int? qualityJPEG,
    int? qualityPNG,
    int? qualityWEBP,
    int? qualityGIF,
    bool? enablePostfix,
    String? postfix,
  }) {
    return ConfigData(
      lossy: lossy ?? this.lossy,
      qualityJPEG: qualityJPEG ?? this.qualityJPEG,
      qualityPNG: qualityPNG ?? this.qualityPNG,
      qualityWEBP: qualityWEBP ?? this.qualityWEBP,
      qualityGIF: qualityGIF ?? this.qualityGIF,
      enablePostfix: enablePostfix ?? this.enablePostfix,
      postfix: postfix ?? this.postfix,
    );
  }
}

final configDir = Directory('${Platform.environment['HOME']}/.config/alic');
final configFile = File('${configDir.path}/config.json');

class Config {
  static final signals.Signal<ConfigData> signal =
      signals.signal(ConfigData.fromJson(const {}));

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
    configFile.writeAsStringSync(jsonEncode(configData.toJson()));
  }

  static void ensureConfigExists() {
    if (!configDir.existsSync()) {
      configDir.createSync(recursive: true);
    }
    if (!configFile.existsSync() || configFile.lengthSync() == 0) {
      configFile.writeAsStringSync('{}', flush: true);
    }
  }

  static void reset() {
    var config = ConfigData.fromJson(const {});
    writeConfig(config);
    signal.value = config;
  }
}
