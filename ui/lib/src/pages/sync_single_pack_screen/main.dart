import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/pages/download_single_pack_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/file_change.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diff.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/diff_addon_tile.dart';
import 'package:pamm_ui/src/widgets/progress_reporter.dart';

class SyncSinglePackScreen extends StatefulWidget {
  SyncSinglePackScreen(this.packName, this.repoPath, {super.key});

  final String packName;
  final String repoPath;
  final ProgressReporterService progressReporterService =
      ProgressReporterService();

  @override
  State<SyncSinglePackScreen> createState() => _SyncSinglePackScreenState();
}

class _SyncSinglePackScreenState extends State<SyncSinglePackScreen> {
  bool isSyncing = false;
  bool isDoneSyncing = false;
  String? error;
  DiffResult? diffResult;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _startSync();
    });
  }

  Future<void> _startSync() async {
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
        clearCache: debugSettingsService.alwaysForceRefresh,
      );
      if (!mounted) return;
      setState(() {
        diffResult = diff;
        isSyncing = false;
        isDoneSyncing = true;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        isSyncing = false;
        error = e.toString();
        isDoneSyncing = true;
      });

      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text("Error checking for updates: $e")),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Updating ${widget.packName} (single)')),
      body: Padding(
        padding: EdgeInsetsGeometry.all(8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: EdgeInsetsGeometry.directional(bottom: 8),
              child: Center(
                child: isDoneSyncing && (diffResult?.hasChanges ?? false)
                    ? _buildDownloadButton(context)
                    : const SizedBox.shrink(),
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
            if (isDoneSyncing) ...[
              if (diffResult?.hasChanges ?? false)
                ..._buildDiffResult()
              else
                const Center(
                  child: Text(
                    'No changes found',
                    style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
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
    );
  }

  Widget _buildDownloadButton(BuildContext context) => ElevatedButton(
    onPressed: () {
      Navigator.of(context).pushReplacement(
        MaterialPageRoute(
          builder: (context) => DownloadSinglePackScreen(
            widget.packName,
            widget.repoPath,
            diffResult!,
          ),
        ),
      );
    },
    child: Text("Download"),
  );

  List<Widget> _buildDiffResult() {
    final diff = diffResult!;
    return [
      Text('Total Download Size: ${format(diff.totalDlSize.toInt())}'),
      Text('Total Size change: ${format(diff.totalSizeChange.toInt())}'),
      Text('Changed addons: ${diff.changeCount}'),
      Expanded(
        flex: 2,
        child: ListView(
          children: diff.fileChanges.entries
              .where((element) => element.value.isNotEmpty)
              .map((entry) => DiffAddonTile(addonName: entry.key, changes: entry.value))
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
