import 'dart:io';

import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/settings_screen/customization_group.dart';
import 'package:pamm_ui/src/pages/settings_screen/debug_group.dart';

import 'arma_group.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Settings"), elevation: 1),
      body: ListView(
        padding: EdgeInsets.symmetric(vertical: 8),
        children: [
          CustomizationGroup(),
          DebugGroup(),
          if (Platform.isWindows) ArmaGroup(),
        ],
      ),
    );
  }
}
