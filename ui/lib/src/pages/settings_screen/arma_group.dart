import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/settings_screen/settings_group.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';
import 'package:pamm_ui/src/services/settings_service.dart';

class ArmaGroup extends StatefulWidget {
  const ArmaGroup({super.key});

  @override
  State<ArmaGroup> createState() => _ArmaGroupState();
}

class _ArmaGroupState extends State<ArmaGroup> {
  @override
  Widget build(BuildContext context) {
    final settings = settingsService.settings.armaSettings;

    return SettingsGroup(
      title: "Arma",
      children: [
        ListTile(
          leading: Icon(Icons.rocket_launch),
          title: Text("Launch mode"),
          trailing: SegmentedButton<LaunchType>(
            segments: const [
              ButtonSegment(
                value: LaunchType.steam,
                label: SizedBox(width: 40, child: Text("Steam")),
                tooltip: "Launch Arma via Steam directly",
              ),
              ButtonSegment(
                value: LaunchType.file,
                label: SizedBox(width: 40, child: Text("File")),
                tooltip:
                    "Launch Arma directly via its executable file. Requires steam to be installed. Might not work as expected if steam is not running",
              ),
            ],
            selected: {settings.launchType},
            onSelectionChanged: (selection) async {
              await settingsService.update(
                (settings) =>
                    settings.armaSettings.launchType = selection.first,
              );
            },
          ),
        ),
      ],
    );
  }
}
