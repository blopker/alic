import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:package_info_plus/package_info_plus.dart';
import 'package:url_launcher/url_launcher.dart';

import 'strings.dart';

@immutable
class Update {
  final String version;
  final int buildNumber;
  final String url;

  const Update(
      {required this.version, required this.url, required this.buildNumber});

  void open() {
    launchUrl(Uri.parse(url));
  }
}

Future<Update> getLatestBuildNumber() async {
  final uri = Uri.parse(Strings.githubAPI);
  final response =
      await http.get(uri, headers: {"Accept": "application/vnd.github+json"});

  if (response.statusCode == 200) {
    final List decodedData = jsonDecode(response.body);
    final tags = decodedData.map((e) => e['name'].toString()).toList();
    final latestBuild = tags.firstWhere((element) => element.contains('+'));
    final buildNumber = int.parse(latestBuild.split('+').last);
    return Update(
      version: latestBuild,
      buildNumber: buildNumber,
      url: Strings.downloadURL,
    );
  } else {
    throw Exception('Failed to fetch tags: ${response.statusCode}');
  }
}

Future<Update?> checkForUpdate({bool force = false}) async {
  // 1. Get your current build number
  PackageInfo packageInfo = await PackageInfo.fromPlatform();
  int currentBuildNumber = int.parse(packageInfo.buildNumber);

  // 2. Get the latest build number
  final update = await getLatestBuildNumber();
  int latestBuildNumber = update.buildNumber;

  // 3. Compare
  if (latestBuildNumber > currentBuildNumber || force) {
    debugPrint('Update available: ${update.version}');
    return update;
  } else {
    debugPrint('You have the latest version');
    return null;
  }
}

void main() async {
  final buildNumber = await getLatestBuildNumber();
  print('Latest Build Number: $buildNumber');
}
