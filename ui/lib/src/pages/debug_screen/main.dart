import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:pamm_ui/main.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';

class DebugScreen extends StatefulWidget {
  const DebugScreen({super.key});

  @override
  State<DebugScreen> createState() => _DebugScreenState();
}

class _DebugScreenState extends State<DebugScreen> {
  var logs = rustLogService.logBuffer;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Debug"), elevation: 1),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Wrap(
              spacing: 16,
              runSpacing: 8,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: [
                DropdownMenu(
                  dropdownMenuEntries:
                      ["error", "warn", "info", "debug", "trace"]
                          .map(
                            (level) =>
                                DropdownMenuEntry(value: level, label: level),
                          )
                          .toList(),
                  requestFocusOnTap: false,
                  initialSelection: rustLogService.currentLogLevel,
                  label: Text('Log Level'),
                  onSelected: (level) {
                    if (level == null) return;
                    rustLogService.setLogLevel(level);
                    setState(() {
                      logs = rustLogService.logBuffer;
                    });
                  },
                ),
                TextButton(
                  onPressed: () {
                    var logText = logs.join('\n');
                    Clipboard.setData(ClipboardData(text: logText));
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text("Logs copied to clipboard")),
                    );
                  },
                  child: Text('Copy Logs to Clipboard'),
                ),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text("Always Force Refresh Packs"),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Switch(
                        value: debugSettingsService.alwaysForceRefresh,
                        onChanged: (val) {
                          setState(() {
                            debugSettingsService.alwaysForceRefresh = val;
                          });
                        },
                      ),
                    ),
                  ],
                ),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text("Use legacy (single-pack) sync"),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Switch(
                        value: debugSettingsService.useLegacySinglePackSync,
                        onChanged: (val) {
                          setState(() {
                            debugSettingsService.useLegacySinglePackSync = val;
                          });
                        },
                      ),
                    ),
                  ],
                ),
                FutureBuilder(
                  future: PackageInfo.fromPlatform(),
                  builder: (context, snapshot) {
                    if (snapshot.connectionState == ConnectionState.waiting) {
                      return SizedBox.shrink();
                    }
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    }
                    var packageInfo = snapshot.data!;
                    return Text(
                      'Version: ${packageInfo.version}+${packageInfo.buildNumber}',
                      style: TextStyle(fontFamily: 'monospace'),
                    );
                  },
                ),
              ],
            ),
          ),
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(12.0),
              child: ListView.builder(
                itemBuilder: (context, index) {
                  var log = logs[index];
                  return Container(
                    color: index % 2 == 0
                        ? Colors.grey.shade200
                        : Colors.transparent,
                    width: double.infinity,
                    child: Text(
                      log,
                      style: TextStyle(fontFamily: 'monospace', fontSize: 12),
                    ),
                  );
                },
                itemCount: logs.length,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
