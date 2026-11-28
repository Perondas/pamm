import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/cancellation.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diffs_with_parents.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/sync_pack_with_parents.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/download_actions.dart';
import 'package:pamm_ui/src/widgets/progress_reporter.dart';

class DownloadScreen extends StatefulWidget {
  DownloadScreen(
    this.packName,
    this.repoPath,
    this.multiDiffResult, {
    super.key,
  });

  final String packName;
  final String repoPath;
  final MultiDiffResult multiDiffResult;

  final ProgressReporterService progressReporterService =
      ProgressReporterService();

  @override
  State<DownloadScreen> createState() => _DownloadScreenState();
}

class _DownloadScreenState extends State<DownloadScreen> {
  final DownloadCancelToken cancelToken = createDownloadCancelToken();
  DownloadState state = DownloadState.running;
  String? error;

  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance.addPostFrameCallback((_) async {
      try {
        final outcome = await syncPackWithParents(
          repoPath: widget.repoPath,
          dartProgressReporter:
              widget.progressReporterService.underlyingReporter,
          diffs: widget.multiDiffResult.results.map((r) => r.diff).toList(),
          cancelToken: cancelToken,
        );

        if (!mounted) return;
        setState(() {
          state = outcome == DownloadOutcome.cancelled
              ? DownloadState.cancelled
              : DownloadState.completed;
        });
      } catch (e) {
        if (!mounted) return;

        setState(() {
          error = e.toString();
          state = DownloadState.failed;
        });

        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text("Error downloading pack: $e")));
      }
    });
  }

  Future<void> _requestCancel() async {
    if (state != DownloadState.running) return;
    final cancelled = await confirmCancelDownload(context, cancelToken);
    if (!cancelled || !mounted) return;
    // Only advance if the download did not finish while the dialog was open.
    if (state == DownloadState.running) {
      setState(() => state = DownloadState.cancelling);
    }
  }

  bool get _isStopped =>
      state != DownloadState.running && state != DownloadState.cancelling;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return PopScope(
      canPop: _isStopped,
      // Leaving mid-download is the same as cancelling it.
      onPopInvokedWithResult: (didPop, _) {
        if (!didPop) _requestCancel();
      },
      child: Scaffold(
        appBar: AppBar(
          title: Text('Downloading ${widget.packName}'),
          automaticallyImplyLeading: false,
        ),
        body: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 960),
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 16, 24, 24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Row(
                    children: [
                      if (_statusIcon(theme) case final icon?) ...[
                        icon,
                        const SizedBox(width: 16),
                      ],
                      Expanded(
                        child: Text(
                          _statusText,
                          style: theme.textTheme.headlineSmall,
                        ),
                      ),
                      DownloadActions(
                        state: state,
                        error: error,
                        onCancel: _requestCancel,
                      ),
                    ],
                  ),
                  const SizedBox(height: 24),
                  Expanded(
                    child: ProgressReporter(widget.progressReporterService),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  String get _statusText => switch (state) {
    DownloadState.running => 'Downloading ${widget.packName}…',
    DownloadState.cancelling => 'Cancelling…',
    DownloadState.completed => 'Download complete',
    DownloadState.cancelled => 'Download cancelled',
    DownloadState.failed => 'Download failed',
  };

  /// Icon for a finished download; none while it is still running.
  Widget? _statusIcon(ThemeData theme) => switch (state) {
    DownloadState.running || DownloadState.cancelling => null,
    DownloadState.completed => const Icon(
      Icons.check_circle,
      size: 32,
      color: Colors.green,
    ),
    DownloadState.cancelled => Icon(
      Icons.cancel_outlined,
      size: 32,
      color: theme.colorScheme.onSurfaceVariant,
    ),
    DownloadState.failed => Icon(
      Icons.error_outline,
      size: 32,
      color: theme.colorScheme.error,
    ),
  };
}
