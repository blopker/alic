import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:package_info_plus/package_info_plus.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:alic/log.dart';

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
  var url = Strings.githubAPI;
  final uri = Uri.parse(url);
  final response =
      await http.get(uri, headers: {"Accept": "application/vnd.github+json"});
  // log.d('Response: ${response.body}');
  if (response.statusCode == 200) {
    final Map<String, dynamic> decodedData = jsonDecode(response.body);
    final latestBuild = decodedData['tag_name'];
    final buildNumber = int.parse(latestBuild.split('+').last);
    log.d('Latest build: $latestBuild, build number: $buildNumber');
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
    log.d('Update available: ${update.version}');
    return update;
  } else {
    log.d('You have the latest version');
    return null;
  }
}

void main() async {
  final buildNumber = await getLatestBuildNumber();
  // ignore: avoid_print
  log.d('Latest Build Number: $buildNumber');
}
