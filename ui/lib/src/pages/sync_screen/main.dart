import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/pages/download_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diff.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diffs_with_parents.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/diff_addon_tile.dart';
import 'package:pamm_ui/src/widgets/progress_reporter.dart';
import 'package:pamm_ui/src/widgets/workshop_addons_notice.dart';

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
        dartProgressReporter: widget.progressReporterService.underlyingReporter,
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

      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text("Error checking for updates: $e")));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Updating ${widget.packName}')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 960),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(24, 16, 24, 24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                if (!isDoneSyncing) ...[
                  _StatusHeader(
                    icon: const SizedBox.square(
                      dimension: 28,
                      child: CircularProgressIndicator(strokeWidth: 3),
                    ),
                    title: 'Checking for updates…',
                  ),
                  const SizedBox(height: 24),
                  Expanded(
                    child: ProgressReporter(widget.progressReporterService),
                  ),
                ] else ...[
                  if (error != null) _buildError(context),
                  if (error == null)
                    WorkshopAddonsNotice(
                      repoPath: widget.repoPath,
                      packName: widget.packName,
                    ),
                  if (multiDiffResult?.hasChanges ?? false)
                    ..._buildDiffResult(context)
                  else if (error == null)
                    Expanded(child: _buildUpToDate(context)),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildError(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Card(
      color: colors.errorContainer,
      child: ListTile(
        leading: Icon(Icons.error_outline, color: colors.onErrorContainer),
        title: Text(
          'Error checking for updates',
          style: TextStyle(color: colors.onErrorContainer),
        ),
        subtitle: Text(
          error!,
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(color: colors.onErrorContainer),
        ),
        trailing: IconButton(
          tooltip: 'Show error',
          onPressed: () {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text("Error checking for updates: $error")),
            );
          },
          icon: const Icon(Icons.bug_report),
          color: Colors.red,
        ),
      ),
    );
  }

  Widget _buildUpToDate(BuildContext context) {
    final theme = Theme.of(context);
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            Icons.check_circle_outline,
            size: 72,
            color: theme.colorScheme.primary,
          ),
          const SizedBox(height: 16),
          Text('No changes found', style: theme.textTheme.headlineSmall),
          const SizedBox(height: 4),
          Text(
            '${widget.packName} is up to date',
            style: theme.textTheme.bodyMedium?.copyWith(
              color: theme.colorScheme.onSurfaceVariant,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildDownloadButton(BuildContext context) {
    final theme = Theme.of(context);
    return FilledButton.icon(
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
      icon: const Icon(Icons.download, size: 28),
      label: const Text("Download"),
      style: FilledButton.styleFrom(
        minimumSize: const Size(220, 60),
        elevation: 4,
        shadowColor: theme.colorScheme.shadow,
        shape: const StadiumBorder(),
        textStyle: theme.textTheme.titleLarge?.copyWith(
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  List<Widget> _buildDiffResult(BuildContext context) {
    final theme = Theme.of(context);
    final multi = multiDiffResult!;
    return [
      Card(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Wrap(
            alignment: WrapAlignment.spaceBetween,
            crossAxisAlignment: WrapCrossAlignment.center,
            spacing: 32,
            runSpacing: 16,
            children: [
              Wrap(
                spacing: 40,
                runSpacing: 16,
                children: [
                  _Stat(
                    label: 'Download size',
                    value: format(multi.totalDlSize.toInt()),
                  ),
                  _Stat(
                    label: 'Size change',
                    value: format(multi.totalSizeChange.toInt()),
                  ),
                  _Stat(
                    label: 'Changed packs',
                    value: '${multi.changedPacks} / ${multi.results.length}',
                  ),
                ],
              ),
              _buildDownloadButton(context),
            ],
          ),
        ),
      ),
      const SizedBox(height: 24),
      Text('Changes', style: theme.textTheme.titleMedium),
      const SizedBox(height: 8),
      Expanded(
        child: Card.outlined(
          clipBehavior: Clip.antiAlias,
          child: ListView(
            children: multi.results
                .map((result) => _buildPackTile(result))
                .toList(),
          ),
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
          .map(
            (entry) =>
                DiffAddonTile(addonName: entry.key, changes: entry.value),
          )
          .toList(),
    );
  }
}

/// A large icon + title line shown at the top of the update screens.
class _StatusHeader extends StatelessWidget {
  const _StatusHeader({required this.icon, required this.title});

  final Widget icon;
  final String title;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        icon,
        const SizedBox(width: 16),
        Text(title, style: Theme.of(context).textTheme.headlineSmall),
      ],
    );
  }
}

/// A labelled figure in the update summary.
class _Stat extends StatelessWidget {
  const _Stat({required this.label, required this.value});

  final String label;
  final String value;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: theme.textTheme.labelLarge?.copyWith(
            color: theme.colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          value,
          style: theme.textTheme.headlineSmall?.copyWith(
            fontWeight: FontWeight.w600,
          ),
        ),
      ],
    );
  }
}
