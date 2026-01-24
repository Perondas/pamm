import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/sync_pack/file_change.dart';
import 'package:ui/src/rust/api/commands/sync_pack/get_diff.dart';
import 'package:ui/src/services/progress_reporter_service.dart';
import 'package:ui/src/widgets/progress_reporter.dart';

class SyncScreen extends StatefulWidget {
  SyncScreen(this.packName, this.repoPath, {super.key});

  final String packName;

  final String repoPath;
  final ProgressReporterService progressReporterService =
      ProgressReporterService();

  @override
  State<SyncScreen> createState() => _SyncScreenState();
}

class _SyncScreenState extends State<SyncScreen> {
  bool isSyncing = false;
  DiffResult? diffResult;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Synchronization Progress')),
      body: Column(
        children: [
          TextButton(
            onPressed: () async {
              if (isSyncing) return;
              setState(() {
                isSyncing = true;
              });
              var diff = await getDiff(
                packName: widget.packName,
                repoPath: widget.repoPath,
                dartProgressReporter:
                    widget.progressReporterService.underlyingReporter,
              );
              setState(() {
                diffResult = diff;
                isSyncing = false;
              });
            },
            child: const Text('Start Sync'),
          ),
          ProgressReporter(widget.progressReporterService),
          if (diffResult?.hasChanges ?? false) ...[
            const Text('Diff Result:'),
            Text('Total Download Size: ${diffResult?.totalChangeSize} bytes'),
            Text('Change Count: ${diffResult?.changeCount}'),
            Expanded(
              child: ListView(
                children: diffResult!.fileChanges.entries.map((entry) {
                  return ExpansionTile(
                    title: Text('Addon: ${entry.key}'),
                    children: entry.value.map((fileChange) {
                      return ListTile(
                        title: Text('File: ${fileChange.filePath}'),
                        subtitle: switch (fileChange.change) {
                          ChangeType_Created(:final size) => Text(
                            'Created - Size: $size bytes',
                          ),
                          ChangeType_Deleted() => const Text('Deleted'),
                          ChangeType_Modified(
                            :final dlSize,
                            :final sizeChange,
                          ) =>
                            Text(
                              'Modified - Size Change: $sizeChange bytes, Download Size: $dlSize bytes',
                            ),
                        },
                      );
                    }).toList(),
                  );
                }).toList(),
              ),
            ),
          ],
        ],
      ),
    );
  }
}
