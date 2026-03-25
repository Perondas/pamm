import 'package:flutter/material.dart';
import 'package:ui/main.dart';
import 'package:flutter/services.dart';


class LogScreen extends StatefulWidget {
  const LogScreen({super.key});

  @override
  State<LogScreen> createState() => _LogScreenState();
}

class _LogScreenState extends State<LogScreen> {
  var logs = rustLogService.logBuffer;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Logs"), elevation: 1),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                DropdownMenu(
                  dropdownMenuEntries: ["error", "warn", "info", "debug", "trace"]
                      .map((level) => DropdownMenuEntry(value: level, label: level))
                      .toList(),
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
                TextButton(onPressed: () {
                  var logText = logs.join('\n');
                  Clipboard.setData(ClipboardData(text: logText));
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(content: Text("Logs copied to clipboard")),
                  );
                }
                  , child: Text('Copy to Clipboard')),
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
