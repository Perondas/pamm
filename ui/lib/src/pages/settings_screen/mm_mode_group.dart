import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/settings_screen/settings_group.dart';
import 'package:pamm_ui/src/services/settings_service.dart';

class MmModeGroup extends StatefulWidget {
  const MmModeGroup({super.key});

  @override
  State<MmModeGroup> createState() => _MmModeGroupState();
}

class _MmModeGroupState extends State<MmModeGroup> {
  @override
  Widget build(BuildContext context) {
    final settings = settingsService.settings.mmSettings;

    return SettingsGroup(
      title: "Mission Maker Mode",
      children: [
        ListTile(
          leading: Icon(Icons.developer_mode),
          title: Text("Enabled"),
          trailing: Switch(
            value: settings.mmModeEnabled,
            onChanged: (value) async {
              await settingsService.update(
                (settings) => settings.mmSettings.mmModeEnabled = value,
              );
            },
          ),
        ),
      ],
    );
  }
}
