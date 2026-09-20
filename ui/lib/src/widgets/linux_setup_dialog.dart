import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:pamm_ui/src/rust/api/commands/linux_setup.dart';

/// Shows the one-time Steam setup this repo needs on Linux.
///
/// pamm never writes Steam's configuration and never runs `flatpak` — the user
/// applies these themselves. Everything is computed when the dialog opens, so
/// it cannot go stale.
class LinuxSetupDialog extends StatefulWidget {
  const LinuxSetupDialog({required this.repoPath, super.key});

  final String repoPath;

  @override
  State<LinuxSetupDialog> createState() => _LinuxSetupDialogState();
}

class _LinuxSetupDialogState extends State<LinuxSetupDialog> {
  LinuxSetupInfo? _setup;
  Object? _error;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final setup = await linuxSetupInfo(repoDir: widget.repoPath);
      if (!mounted) return;
      setState(() => _setup = setup);
    } catch (e) {
      if (!mounted) return;
      setState(() => _error = e);
    }
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Steam setup"),
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
    final error = _error;
    if (error != null) {
      return Text(
        "Could not work out the Steam setup: $error",
        style: TextStyle(color: Theme.of(context).colorScheme.error),
      );
    }

    final setup = _setup;
    if (setup == null) {
      return const Center(child: Padding(
        padding: EdgeInsets.all(24),
        child: CircularProgressIndicator(),
      ));
    }

    final flavour = setup.steamFlavour == SteamFlavour.flatpak
        ? "Flatpak"
        : "native";

    final nothingToDo =
        setup.launchOptions == null && setup.flatpakOverrideCommand == null;

    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text("$flavour Steam · ${setup.armaInstallDir}",
              style: Theme.of(context).textTheme.bodySmall),
          const SizedBox(height: 16),
          if (nothingToDo)
            const Text(
              "Nothing to set up — Steam can already reach this repo's mods.",
            ),
          if (setup.launchOptions != null)
            _CopyableCommand(
              label:
                  "Paste into Steam → Arma 3 → Properties → Launch Options. "
                  "Steam can stay open.",
              command: setup.launchOptions!,
            ),
          if (setup.flatpakOverrideCommand != null) ...[
            if (setup.launchOptions != null) const SizedBox(height: 16),
            _CopyableCommand(
              label:
                  "This repo's mods sit outside the Flatpak sandbox, so Steam "
                  "also needs access to them. Run this, then restart Steam.",
              command: setup.flatpakOverrideCommand!,
            ),
          ],
        ],
      ),
    );
  }
}

class _CopyableCommand extends StatelessWidget {
  const _CopyableCommand({required this.label, required this.command});

  final String label;
  final String command;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: theme.textTheme.bodyMedium),
        const SizedBox(height: 8),
        Container(
          width: double.infinity,
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: theme.colorScheme.surfaceContainerHighest,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: SelectableText(
                  command,
                  style: theme.textTheme.bodySmall?.copyWith(
                    fontFamily: "monospace",
                  ),
                ),
              ),
              IconButton(
                tooltip: "Copy",
                icon: const Icon(Icons.copy, size: 18),
                onPressed: () async {
                  await Clipboard.setData(ClipboardData(text: command));
                  if (!context.mounted) return;
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text("Copied")),
                  );
                },
              ),
            ],
          ),
        ),
      ],
    );
  }
}
