import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/debug_screen/main.dart';
import 'package:pamm_ui/src/pages/settings_screen/settings_group.dart';

class DebugGroup extends StatelessWidget {
  const DebugGroup({super.key});

  @override
  Widget build(BuildContext context) {
    return SettingsGroup(
      title: "Debug",
      children: [
        ListTile(
          leading: Icon(Icons.bug_report),
          title: Text("Debug tools"),
          subtitle: Text("Logs, log level and debug switches"),
          trailing: Icon(Icons.chevron_right),
          onTap: () {
            Navigator.of(
              context,
            ).push(MaterialPageRoute(builder: (context) => DebugScreen()));
          },
        ),
      ],
    );
  }
}
