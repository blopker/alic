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

  const ConfigData({
    required this.lossy,
    required this.qualityJPEG,
    required this.qualityPNG,
    required this.qualityWEBP,
    required this.qualityGIF,
  });

  ConfigData.fromJson(Map<String, dynamic> json)
      : lossy = json['lossy'] ?? true,
        qualityJPEG = json['qualityJPEG'] ?? 80,
        qualityPNG = json['qualityPNG'] ?? 80,
        qualityWEBP = json['qualityWEBP'] ?? 60,
        qualityGIF = json['qualityGIF'] ?? 80;

  Map<String, dynamic> toJson() => {
        'lossy': lossy,
        'qualityJPEG': qualityJPEG,
        'qualityPNG': qualityPNG,
        'qualityWEBP': qualityWEBP,
        'qualityGIF': qualityGIF,
      };

  ConfigData copyWith({
    bool? lossy,
    int? qualityJPEG,
    int? qualityPNG,
    int? qualityWEBP,
    int? qualityGIF,
  }) {
    return ConfigData(
      lossy: lossy ?? this.lossy,
      qualityJPEG: qualityJPEG ?? this.qualityJPEG,
      qualityPNG: qualityPNG ?? this.qualityPNG,
      qualityWEBP: qualityWEBP ?? this.qualityWEBP,
      qualityGIF: qualityGIF ?? this.qualityGIF,
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
      writeConfig(signal.value);
    });
  }

  static ConfigData readConfig() {
    ensureConfigExists();
    var configData = configFile.readAsStringSync();
    return ConfigData.fromJson(jsonDecode(configData));
  }

  static void writeConfig(ConfigData configData) {
    ensureConfigExists();
    configFile.writeAsStringSync(jsonEncode(configData.toJson()));
  }

  static void ensureConfigExists() {
    if (!configDir.existsSync()) {
      configDir.createSync();
    }
    if (!configFile.existsSync()) {
      configFile.writeAsStringSync('{}');
    }
  }

  static void reset() {
    var config = ConfigData.fromJson(const {});
    writeConfig(config);
    signal.value = config;
  }
}
