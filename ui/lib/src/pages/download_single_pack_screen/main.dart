import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/cancellation.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/get_diff.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/sync_pack.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';
import 'package:pamm_ui/src/widgets/download_actions.dart';
import 'package:pamm_ui/src/widgets/progress_reporter.dart';

class DownloadSinglePackScreen extends StatefulWidget {
  DownloadSinglePackScreen(
    this.packName,
    this.repoPath,
    this.diffResult, {
    super.key,
  });

  final String packName;
  final String repoPath;
  final DiffResult diffResult;

  final ProgressReporterService progressReporterService =
      ProgressReporterService();

  @override
  State<DownloadSinglePackScreen> createState() =>
      _DownloadSinglePackScreenState();
}

class _DownloadSinglePackScreenState extends State<DownloadSinglePackScreen> {
  final DownloadCancelToken cancelToken = createDownloadCancelToken();
  DownloadState state = DownloadState.running;
  String? error;

  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance.addPostFrameCallback((_) async {
      try {
        final outcome = await syncPack(
          packName: widget.packName,
          repoPath: widget.repoPath,
          dartProgressReporter:
              widget.progressReporterService.underlyingReporter,
          diff: widget.diffResult.diff,
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
    return PopScope(
      canPop: _isStopped,
      // Leaving mid-download is the same as cancelling it.
      onPopInvokedWithResult: (didPop, _) {
        if (!didPop) _requestCancel();
      },
      child: Scaffold(
        appBar: AppBar(
          title: Text('Downloading ${widget.packName} (single)'),
          automaticallyImplyLeading: false,
        ),
        body: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              DownloadActions(
                state: state,
                error: error,
                onCancel: _requestCancel,
              ),
              Expanded(child: ProgressReporter(widget.progressReporterService)),
            ],
          ),
        ),
      ),
    );
  }
}
