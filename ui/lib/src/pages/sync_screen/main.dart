import 'package:flutter/material.dart';
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
  String diffResult = '';

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
          if (!isSyncing) ...[
            Expanded(child: SingleChildScrollView(child: Text(diffResult))),
          ],
        ],
      ),
    );
  }
}
