import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/pages/download_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diff.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diffs_with_parents.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/diff_addon_tile.dart';
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
  MultiDiffResult? multiDiffResult;

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
      var diff = await getDiffWithParents(
        packName: widget.packName,
        repoPath: widget.repoPath,
        dartProgressReporter:
            widget.progressReporterService.underlyingReporter,
        clearCache: debugSettingsService.alwaysForceRefresh,
      );
      if (!mounted) return;
      setState(() {
        multiDiffResult = diff;
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
      appBar: AppBar(title: Text('Updating ${widget.packName}')),
      body: Padding(
        padding: EdgeInsetsGeometry.all(8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: EdgeInsetsGeometry.directional(bottom: 8),
              child: Center(
                child: isDoneSyncing && (multiDiffResult?.hasChanges ?? false)
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
              if (multiDiffResult?.hasChanges ?? false)
                ..._buildDiffResult()
              else
                const Center(
                  child: Text(
                    'No changes found',
                    style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                  ),
                ),
            ] else ...[
              Expanded(
                flex: 1,
                child: Padding(
                  padding: EdgeInsetsGeometry.all(8),
                  child: ProgressReporter(widget.progressReporterService),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildDownloadButton(BuildContext context) => ElevatedButton(
    onPressed: () {
      Navigator.of(context).pushReplacement(
        MaterialPageRoute(
          builder: (context) => DownloadScreen(
            widget.packName,
            widget.repoPath,
            multiDiffResult!,
          ),
        ),
      );
    },
    child: Text("Download"),
  );

  List<Widget> _buildDiffResult() {
    final multi = multiDiffResult!;
    return [
      Text('Total Download Size: ${format(multi.totalDlSize.toInt())}'),
      Text('Total Size change: ${format(multi.totalSizeChange.toInt())}'),
      Text('Changed packs: ${multi.changedPacks} / ${multi.results.length}'),
      Expanded(
        flex: 2,
        child: ListView(
          children: multi.results
              .map((result) => _buildPackTile(result))
              .toList(),
        ),
      ),
    ];
  }

  Widget _buildPackTile(DiffResult result) {
    if (!result.hasChanges) {
      return ListTile(
        leading: const Icon(Icons.check_circle_outline),
        title: Text(result.packName),
        subtitle: const Text('No changes'),
      );
    }

    final addonEntries = result.fileChanges.entries
        .where((entry) => entry.value.isNotEmpty)
        .toList();

    return ExpansionTile(
      leading: const Icon(Icons.inventory_2_outlined),
      title: Text(result.packName),
      subtitle: Row(
        children: [
          Text('${result.changeCount} changes'),
          const SizedBox(width: 16),
          Text('Download size: ${format(result.totalDlSize.toInt())}'),
        ],
      ),
      children: addonEntries
          .map((entry) => DiffAddonTile(addonName: entry.key, changes: entry.value))
          .toList(),
    );
  }
}
