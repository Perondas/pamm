import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/workshop_addons.dart';
import 'package:url_launcher/url_launcher.dart';

const _explanation =
    'These addons are not downloaded by pamm. Subscribe to them on the Steam '
    'workshop, and let Steam download them before launching.';

/// Points the user at the workshop addons the pack needs but that cannot be
/// found. Renders nothing while loading, or when every addon was found.
class WorkshopAddonsNotice extends StatefulWidget {
  const WorkshopAddonsNotice({
    super.key,
    required this.repoPath,
    required this.packName,
  });

  final String repoPath;
  final String packName;

  @override
  State<WorkshopAddonsNotice> createState() => _WorkshopAddonsNoticeState();
}

class _WorkshopAddonsNoticeState extends State<WorkshopAddonsNotice> {
  List<WorkshopAddonNotFound>? _notFound;
  String? _error;
  bool _checking = false;

  @override
  void initState() {
    super.initState();
    _check();
  }

  Future<void> _check() async {
    setState(() => _checking = true);
    try {
      final notFound = await workshopAddonsNotFound(
        repoPath: widget.repoPath,
        packName: widget.packName,
      );
      if (!mounted) return;
      setState(() {
        _notFound = notFound;
        _error = null;
        _checking = false;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _error = e.toString();
        _checking = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final error = _error;
    final notFound = _notFound;

    if (error != null) {
      return Card(
        child: ListTile(
          leading: const Icon(Icons.error_outline, color: Colors.red),
          title: const Text('Failed to check workshop addons'),
          subtitle: Text(error),
          trailing: _buildCheckAgainButton(),
        ),
      );
    }

    if (notFound == null || notFound.isEmpty) {
      return const SizedBox.shrink();
    }

    return Card(
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.warning_amber, color: Colors.orange),
              title: const Text('Workshop addons not found'),
              subtitle: const Text(_explanation),
              trailing: _buildCheckAgainButton(),
            ),
            ...notFound.map((addon) => WorkshopAddonTile(addon: addon)),
          ],
        ),
      ),
    );
  }

  Widget _buildCheckAgainButton() => TextButton(
    onPressed: _checking ? null : _check,
    child: const Text('Check again'),
  );
}

/// A single workshop addon that was not found, with a link to subscribe.
class WorkshopAddonTile extends StatelessWidget {
  const WorkshopAddonTile({super.key, required this.addon});

  final WorkshopAddonNotFound addon;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      dense: true,
      title: Text(addon.name),
      subtitle: Text('Pack: ${addon.packName}'),
      trailing: IconButton(
        tooltip: 'Open in Steam workshop',
        icon: const Icon(Icons.open_in_new),
        onPressed: () => _openInSteam(addon),
      ),
    );
  }
}

/// Opens the addon's workshop page in the Steam client, falling back to the
/// browser when nothing handles `steam://` links.
Future<void> _openInSteam(WorkshopAddonNotFound addon) async {
  try {
    if (await launchUrl(Uri.parse(addon.steamUrl))) return;
  } catch (_) {
    // No handler for steam:// links, use the browser instead.
  }
  await launchUrl(Uri.parse(addon.url));
}

/// Shows a dialog listing the workshop addons the pack needs but that cannot be
/// found. Does nothing when every addon was found.
Future<void> showWorkshopAddonsDialogIfNeeded(
  BuildContext context, {
  required String repoPath,
  required String packName,
}) async {
  final notFound = await workshopAddonsNotFound(
    repoPath: repoPath,
    packName: packName,
  );

  if (notFound.isEmpty || !context.mounted) return;

  await showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('Workshop addons not found'),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(_explanation),
            const SizedBox(height: 8),
            ...notFound.map((addon) => WorkshopAddonTile(addon: addon)),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    ),
  );
}
