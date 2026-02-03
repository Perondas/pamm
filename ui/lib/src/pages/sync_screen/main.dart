import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:ui/src/pages/download_screen/main.dart';
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
  bool isDoneSyncing = false;
  DiffResult? diffResult;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Updating ${widget.packName}')),
      body: Column(
        children: [
          Center(
            child: isDoneSyncing
                ? _buildDownloadButton(context)
                : _buildSyncButton(),
          ),
          ProgressReporter(widget.progressReporterService),
          if (diffResult?.hasChanges ?? false) ..._buildDiffResult(),
        ],
      ),
    );
  }

  TextButton _buildDownloadButton(BuildContext context) => TextButton(
    onPressed: () {
      Navigator.of(context).pushReplacement(
        MaterialPageRoute(
          builder: (context) =>
              DownloadScreen(widget.packName, widget.repoPath, diffResult!),
        ),
      );
    },
    child: Text("Download"),
  );

  TextButton _buildSyncButton() {
    return TextButton(
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
          clearCache: true,
        );
        if (!mounted) return;
        setState(() {
          diffResult = diff;
          isSyncing = false;
          isDoneSyncing = true;
        });
      },
      child: const Text('Check for update'),
    );
  }

  List<Widget> _buildDiffResult() {
    return [
      Text(
        'Total Download Size: ${format(diffResult?.totalChangeSize.toInt() ?? 0)}',
      ),
      Text('Changed addons: ${diffResult?.changeCount}'),
      Expanded(
        child: ListView(
          children: diffResult!.fileChanges.entries.map((entry) {
            return ExpansionTile(
              title: Text(entry.key),
              subtitle: Row(
                children: [
                  Text('${entry.value.length} changes'),
                  const SizedBox(width: 16),
                  Text(
                    'Download size: ${format(entry.value.fold(0, (previousValue, element) {
                      return sizeChangeToDlSize(element.change) + previousValue;
                    }))}',
                  ),
                ],
              ),
              children: entry.value.map((fileChange) {
                return ListTile(
                  title: Text('File: ${fileChange.filePath}'),
                  subtitle: switch (fileChange.change) {
                    ChangeType_Created(:final size) => Text(
                      'Download - Size: ${format(size.toInt())}',
                    ),
                    ChangeType_Deleted() => const Text('Delete'),
                    ChangeType_Modified(:final dlSize, :final sizeChange) => Text(
                      'Patch - Size Change: ${format(sizeChange)}, Download Size: ${format(dlSize.toInt())}',
                    ),
                  },
                );
              }).toList(),
            );
          }).toList(),
        ),
      ),
    ];
  }
}

int sizeChangeToDlSize(ChangeType change) {
  return switch (change) {
    ChangeType_Created(:final size) => size.toInt(),
    ChangeType_Deleted() => 0,
    ChangeType_Modified(:final dlSize) => dlSize.toInt(),
  };
}
