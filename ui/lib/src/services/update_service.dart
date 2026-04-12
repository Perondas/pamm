import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:package_info_plus/package_info_plus.dart';
import 'package:pamm_ui/main.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:version/version.dart';

const releaseUrl = "https://api.github.com/repos/perondas/pamm/releases/latest";

Future<void> checkForUpdates() async {
  var packageInfo = await PackageInfo.fromPlatform();

  var cleanedVersion = Version.parse(packageInfo.version.replaceAll("v", ""));

  var response = await http.get(Uri.parse(releaseUrl));

  if (response.statusCode != 200) {
    debugPrint("Failed to check for updates: ${response.statusCode}");
    return;
  }

  var releaseData = json.decode(response.body) as Map<String, dynamic>;

  var latestVersion = Version.parse(
    (releaseData['tag_name'] as String).replaceAll("v", ""),
  );

  if (latestVersion > cleanedVersion) {
    var releaseUrl = releaseData['html_url'] as String;
    showDialog(
      context: NavigationService.navigatorKey.currentContext!,
      builder: (context) => AlertDialog(
        title: Text("Update Available"),
        content: Text(
          "A new version of PAMM is available. Would you like to download it?",
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: Text("Later"),
          ),
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              launchUrl(Uri.parse(releaseUrl));
            },
            child: Text("Go to release page"),
          ),
        ],
      ),
    );
  }
}
