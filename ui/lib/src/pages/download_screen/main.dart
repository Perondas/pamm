import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/sync_pack/get_diff.dart';
import 'package:ui/src/rust/api/commands/sync_pack/sync_pack.dart';
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
  bool done = false;

  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance.addPostFrameCallback((_) async {
      await syncPack(
        packName: widget.packName,
        repoPath: widget.repoPath,
        dartProgressReporter: widget.progressReporterService.underlyingReporter,
        packDiff: widget.diffResult.diff,
      );

      if (!mounted) return;
      setState(() {});
      done = true;
    });
  }

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
            if (done)
              TextButton(
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: const Text("Done"),
              ),
            ProgressReporter(widget.progressReporterService),
          ],
        ),
      ),
    );
  }
}
