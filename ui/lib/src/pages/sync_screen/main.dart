import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/pages/download_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/file_change.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diff.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/progress_reporter.dart';

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
  String? error;
  DiffResult? diffResult;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Material(
          elevation: 1,
          child: SizedBox(
            height: kToolbarHeight,
            child: Row(
              children: [
                IconButton(
                  icon: const Icon(Icons.arrow_back),
                  onPressed: () => Navigator.of(context).pop(),
                ),
                Expanded(
                  child: Text(
                    'Updating ${widget.packName}',
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                ),
              ],
            ),
          ),
        ),
        Expanded(
          child: Padding(
            padding: EdgeInsetsGeometry.all(8),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Padding(
                  padding: EdgeInsetsGeometry.directional(bottom: 8),
                  child: Center(
                    child: isDoneSyncing && (diffResult?.hasChanges ?? false)
                        ? _buildDownloadButton(context)
                        : _buildSyncButton(),
                  ),
                ),
                if (error != null) ...[
                  IconButton(
                    onPressed: () {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text("Error checking for updates: $error"),
                        ),
                      );
                    },
                    icon: Icon(Icons.bug_report),
                    color: Colors.red,
                  ),
                ],
                if (diffResult != null) ...[
                  if (diffResult!.hasChanges)
                    ..._buildDiffResult()
                  else
                    const Center(
                      child: Text(
                        'No changes found',
                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)
                      ),
                    ),
                ],
                Expanded(
                  flex: 1,
                  child: Padding(
                    padding: EdgeInsetsGeometry.all(8),
                    child: ProgressReporter(widget.progressReporterService),
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildDownloadButton(BuildContext context) => ElevatedButton(
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

  Widget _buildSyncButton() {
    return ElevatedButton(
      onPressed: () async {
        if (isSyncing) return;
        setState(() {
          isSyncing = true;
        });
        try {
          var diff = await getDiff(
            packName: widget.packName,
            repoPath: widget.repoPath,
            dartProgressReporter:
                widget.progressReporterService.underlyingReporter,
            clearCache: false,
          );
          if (!mounted) return;
          setState(() {
            diffResult = diff;
            isSyncing = false;
            isDoneSyncing = true;
          });
        } catch (e) {
          setState(() {
            isSyncing = false;
            error = e.toString();
          });

          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text("Error checking for updates: $e")),
          );
        }
      },
      child: const Text('Check for updates'),
    );
  }

  List<Widget> _buildDiffResult() {
    return [
      Text(
        'Total Download Size: ${format(diffResult?.totalDlSize.toInt() ?? 0)}',
      ),
      Text(
        'Total Size change: ${format(diffResult?.totalSizeChange.toInt() ?? 0)}',
      ),
      Text('Changed addons: ${diffResult?.changeCount}'),
      Expanded(
        flex: 2,
        child: ListView(
          children: diffResult!.fileChanges.entries
              .where((element) => element.value.isNotEmpty)
              .map((entry) {
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
                        ChangeType_Modified(:final dlSize, :final sizeChange) =>
                          Text(
                            'Patch - Size Change: ${format(sizeChange)}, Download Size: ${format(dlSize.toInt())}',
                          ),
                      },
                    );
                  }).toList(),
                );
              })
              .toList(),
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
