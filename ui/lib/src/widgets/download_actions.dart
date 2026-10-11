import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/cancellation.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';

enum DownloadState { running, cancelling, completed, cancelled, failed }

/// Asks the user to confirm, then cancels the download behind [token].
/// Returns whether the download was cancelled.
Future<bool> confirmCancelDownload(
  BuildContext context,
  DownloadCancelToken token,
) async {
  final confirmed = await showDialog<bool>(
    context: context,
    builder: (_) => const ConfirmDialog(
      title: 'Cancel download?',
      content:
          'Files downloaded so far are kept. The pack will show as out of date '
          'until it is synced again.',
    ),
  );

  if (confirmed != true) return false;

  token.cancel();
  return true;
}

/// The buttons shown above a download's progress: cancel while it runs, and a
/// way back once it has stopped.
class DownloadActions extends StatelessWidget {
  const DownloadActions({
    required this.state,
    required this.onCancel,
    this.error,
    super.key,
  });

  final DownloadState state;
  final VoidCallback onCancel;
  final String? error;

  @override
  Widget build(BuildContext context) {
    switch (state) {
      case DownloadState.running:
        return OutlinedButton.icon(
          onPressed: onCancel,
          style: OutlinedButton.styleFrom(foregroundColor: Colors.red),
          icon: const Icon(Icons.close),
          label: const Text('Cancel download'),
        );
      case DownloadState.cancelling:
        return const OutlinedButton(
          onPressed: null,
          child: Text('Cancelling…'),
        );
      case DownloadState.completed:
        return ElevatedButton(
          onPressed: () => Navigator.of(context).pop(),
          style: ElevatedButton.styleFrom(
            backgroundColor: Colors.green,
            foregroundColor: Colors.white,
          ),
          child: const Text('Done'),
        );
      case DownloadState.cancelled:
        return Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text('Download cancelled'),
            const SizedBox(height: 8),
            ElevatedButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Close'),
            ),
          ],
        );
      case DownloadState.failed:
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            ElevatedButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Close'),
            ),
            IconButton(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Error downloading pack: $error')),
                );
              },
              icon: const Icon(Icons.bug_report, color: Colors.red),
            ),
          ],
        );
    }
  }
}
