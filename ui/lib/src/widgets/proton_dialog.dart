import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/proton.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';

/// Runs Windows programs in Arma's Proton prefix on Linux: installing and
/// launching TeamSpeak, browsing files, and stopping everything.
///
/// The prefix is detected when the dialog opens, and again after an install,
/// as the user can switch Arma's Proton version at any time.
class ProtonDialog extends StatefulWidget {
  const ProtonDialog({super.key});

  @override
  State<ProtonDialog> createState() => _ProtonDialogState();
}

class _ProtonDialogState extends State<ProtonDialog> {
  ProtonInfo? _info;
  Object? _error;
  bool _busy = false;
  bool _interactiveInstall = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final info = await protonInfo();
      if (!mounted) return;
      setState(() {
        _info = info;
        _error = null;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() => _error = e);
    }
  }

  /// Runs an action, reporting failure in a snackbar.
  Future<void> _run(Future<void> Function() action) async {
    setState(() => _busy = true);
    try {
      await action();
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text("$e")));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _installTeamspeak() async {
    final installer = await FilePicker.pickFile(
      dialogTitle: "Select the TeamSpeak 3 installer",
      type: FileType.custom,
      allowedExtensions: ["exe"],
    );
    final path = installer?.path;
    if (path == null) return;

    await _run(() async {
      await protonInstallTeamspeak(
        installer: path,
        silent: !_interactiveInstall,
      );
      await _load();
    });
  }

  Future<void> _killAll() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (_) => const ConfirmDialog(
        title: "Stop everything?",
        content:
            "This stops every program in Arma's Proton prefix, including the game.",
      ),
    );
    if (confirmed != true) return;

    await _run(protonKillAll);
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Proton"),
      content: SizedBox(width: 560, child: _body(context)),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text("Close"),
        ),
      ],
    );
  }

  Widget _body(BuildContext context) {
    final theme = Theme.of(context);

    final error = _error;
    if (error != null) {
      return Text(
        "Could not find Arma's Proton prefix: $error",
        style: TextStyle(color: theme.colorScheme.error),
      );
    }

    final info = _info;
    if (info == null) {
      return const Center(
        child: Padding(
          padding: EdgeInsets.all(24),
          child: CircularProgressIndicator(),
        ),
      );
    }

    final teamspeakExe = info.teamspeakExe;

    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            "Proton ${info.version ?? "(unknown version)"} · ${info.protonDir}",
            style: theme.textTheme.bodySmall,
          ),
          if (_busy) ...[
            const SizedBox(height: 8),
            const LinearProgressIndicator(),
          ],
          const SizedBox(height: 16),
          Text("TeamSpeak", style: theme.textTheme.titleSmall),
          const SizedBox(height: 4),
          Text(
            teamspeakExe == null
                ? "Not installed"
                : "Installed at $teamspeakExe",
            style: theme.textTheme.bodySmall,
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              if (teamspeakExe != null) ...[
                FilledButton.icon(
                  onPressed: _busy ? null : () => _run(protonLaunchTeamspeak),
                  icon: const Icon(Icons.headset_mic),
                  label: const Text("Launch"),
                ),
                const SizedBox(width: 8),
              ],
              OutlinedButton.icon(
                onPressed: _busy ? null : _installTeamspeak,
                icon: const Icon(Icons.download),
                label: Text(teamspeakExe == null ? "Install…" : "Reinstall…"),
              ),
              const SizedBox(width: 8),
              Checkbox(
                value: _interactiveInstall,
                onChanged: _busy
                    ? null
                    : (value) =>
                          setState(() => _interactiveInstall = value ?? false),
              ),
              const Text("Show installer wizard"),
            ],
          ),
          const SizedBox(height: 16),
          Text("Prefix", style: theme.textTheme.titleSmall),
          const SizedBox(height: 8),
          Wrap(
            spacing: 8,
            children: [
              OutlinedButton.icon(
                onPressed: _busy
                    ? null
                    : () => _run(() => protonOpenExplorer()),
                icon: const Icon(Icons.folder_open),
                label: const Text("File browser"),
              ),
              OutlinedButton.icon(
                onPressed: _busy ? null : _killAll,
                style: OutlinedButton.styleFrom(
                  foregroundColor: theme.colorScheme.error,
                ),
                icon: const Icon(Icons.power_settings_new),
                label: const Text("Stop everything"),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
