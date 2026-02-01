import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/sync_pack/get_diff.dart';
import 'package:ui/src/services/progress_reporter_service.dart';
import 'package:ui/src/widgets/progress_reporter.dart';

class DownloadScreen extends StatefulWidget {
  DownloadScreen(this.packName, this.repoPath, this.diffResult, {super.key});

  final String packName;
  final String repoPath;
  final DiffResult diffResult;

  final ProgressReporterService progressReporterService =
      ProgressReporterService();

  @override
  State<DownloadScreen> createState() => _DownloadScreenState();
}

class _DownloadScreenState extends State<DownloadScreen> {
  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: false,
      child: Scaffold(
        appBar: AppBar(
          title: Text('Downloading ${widget.packName}'),
          automaticallyImplyLeading: false,
        ),
        body: Column(
          children: [
            ProgressReporter(widget.progressReporterService),
          ],
        ),
      ),
    );
  }
}
