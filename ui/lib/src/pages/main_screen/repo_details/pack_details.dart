import 'dart:io';

import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_actions.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_state.dart';
import 'package:pamm_ui/src/services/settings_service.dart';
import 'package:pamm_ui/src/util/media.dart';
import 'package:pamm_ui/src/widgets/launch_teamspeak_button.dart';
import 'package:pamm_ui/src/widgets/linux_setup_dialog.dart';

/// Right-hand pane: the repo banner, the play button and the details and
/// actions of the selected pack.
class PackDetails extends StatelessWidget {
  const PackDetails({
    required this.repoState,
    required this.pack,
    required this.onLocalPackDeleted,
    super.key,
  });

  final RepoWithPath repoState;
  final PackStateManager? pack;
  final VoidCallback onLocalPackDeleted;

  @override
  Widget build(BuildContext context) {
    final pack = this.pack;

    return SingleChildScrollView(
      padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 40),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 720),
          child: pack == null
              ? Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    ..._buildHeader(context),
                    const SizedBox(height: 24),
                    const Center(child: Text('Select a pack to view details')),
                  ],
                )
              : ListenableBuilder(
                  listenable: pack,
                  builder: (context, _) => Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      ..._buildHeader(context),
                      const SizedBox(height: 28),
                      _buildPlayRow(context, pack),
                      const SizedBox(height: 20),
                      _buildActions(context, pack),
                      const SizedBox(height: 28),
                      const Divider(),
                      const SizedBox(height: 20),
                      _buildInfo(context, pack),
                    ],
                  ),
                ),
        ),
      ),
    );
  }

  /// The repo's banner, or its name when it has none.
  List<Widget> _buildHeader(BuildContext context) {
    final theme = Theme.of(context);
    final repo = repoState.repo;
    final banner = mediaFile(repoState.path, repo.customization?.banner);

    final title = Text(
      repo.name,
      style: theme.textTheme.headlineMedium?.copyWith(
        fontWeight: FontWeight.w600,
      ),
    );

    if (banner == null) return [title];

    return [
      ClipRRect(
        borderRadius: BorderRadius.circular(4),
        child: Image(
          image: MediaFileImage(banner),
          fit: BoxFit.cover,
          errorBuilder: (_, _, _) => title,
        ),
      ),
    ];
  }

  Widget _buildPlayRow(BuildContext context, PackStateManager pack) {
    final theme = Theme.of(context);
    final mmSettings = settingsService.settings.mmSettings;
    final updateAvailable = pack.upToDate == false;

    // Play and Sync share one prominent style.
    final bigButtonStyle = FilledButton.styleFrom(
      minimumSize: const Size(240, 64),
      elevation: 4,
      shadowColor: theme.colorScheme.shadow,
      shape: const StadiumBorder(),
      textStyle: theme.textTheme.headlineSmall?.copyWith(
        fontWeight: FontWeight.w600,
      ),
    );

    return Column(
      children: [
        Text(pack.packName, style: theme.textTheme.headlineSmall),
        const SizedBox(height: 16),
        Wrap(
          alignment: WrapAlignment.center,
          spacing: 24,
          runSpacing: 16,
          children: [
            Tooltip(
              message: 'Launch ${pack.packName}',
              child: FilledButton.icon(
                onPressed: () => launchPack(context, pack),
                icon: const Icon(Icons.play_arrow, size: 32),
                label: const Text('Play'),
                style: bigButtonStyle,
              ),
            ),
            Tooltip(
              message: updateAvailable ? 'Updates available' : 'Sync pack',
              child: FilledButton.icon(
                onPressed: () => syncPack(context, pack),
                icon: Badge(
                  isLabelVisible: updateAvailable,
                  child: Icon(
                    pack.isLocal ? Icons.sync : Icons.download,
                    size: 32,
                  ),
                ),
                label: Text(updateAvailable ? 'Update' : 'Sync'),
                style: bigButtonStyle,
              ),
            ),
          ],
        ),
        if (mmSettings.mmModeEnabled) ...[
          const SizedBox(height: 12),
          _CheckboxRow(
            label: 'Launch without optionals',
            value: mmSettings.launchWithoutOptionals,
            onChanged: (value) => settingsService.update(
              (settings) => settings.mmSettings.launchWithoutOptionals = value,
            ),
          ),
        ],
      ],
    );
  }

  Widget _buildActions(BuildContext context, PackStateManager pack) {
    final mmMode = settingsService.settings.mmSettings.mmModeEnabled;

    return Wrap(
      alignment: WrapAlignment.center,
      spacing: 8,
      runSpacing: 8,
      children: [
        OutlinedButton.icon(
          onPressed: () => editPack(context, pack),
          icon: const Icon(Icons.settings),
          label: const Text('Pack settings'),
        ),
        OutlinedButton.icon(
          onPressed: () => clearPackContent(context, pack),
          icon: const Icon(Icons.clear),
          label: const Text('Clear from disk'),
        ),
        if (pack.isLocal && mmMode)
          OutlinedButton.icon(
            onPressed: () => deleteLocalPack(context, pack, onLocalPackDeleted),
            icon: const Icon(Icons.delete_forever),
            label: const Text('Delete pack'),
          ),
        if (Platform.isLinux) ...[
          OutlinedButton.icon(
            onPressed: () => showDialog<void>(
              context: context,
              builder: (_) => LinuxSetupDialog(repoPath: repoState.path),
            ),
            icon: const Icon(Icons.settings_suggest),
            label: const Text("Steam setup"),
          ),
          const LaunchTeamspeakButton(),
        ],
      ],
    );
  }

  Widget _buildInfo(BuildContext context, PackStateManager pack) {
    final theme = Theme.of(context);
    final muted = theme.textTheme.bodyMedium?.copyWith(
      color: theme.colorScheme.onSurfaceVariant,
    );

    final packDescription = pack.info?.description ?? pack.display?.description;
    final description = (packDescription?.isNotEmpty ?? false)
        ? packDescription!
        : repoState.repo.description;
    final parents = pack.info?.parents ?? const <String>[];

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Expanded(
          flex: 3,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('About', style: theme.textTheme.titleMedium),
              const SizedBox(height: 8),
              Text(
                description.isEmpty ? 'No description' : description,
                style: muted,
              ),
              if (pack.info?.enabledCdlc != null) ...[
                const SizedBox(height: 12),
                Row(
                  children: [
                    const Icon(Icons.warning_amber, size: 16),
                    const SizedBox(width: 6),
                    Text(
                      'CDLC: ${pack.info!.enabledCdlc!.toString().split('.').last}',
                      style: muted,
                    ),
                  ],
                ),
              ],
            ],
          ),
        ),
        const SizedBox(width: 48),
        Expanded(
          flex: 2,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('Inherits from', style: theme.textTheme.titleMedium),
              const SizedBox(height: 8),
              if (parents.isEmpty) Text('No parent packs', style: muted),
              for (final parent in parents) Text(parent, style: muted),
            ],
          ),
        ),
      ],
    );
  }
}

class _CheckboxRow extends StatelessWidget {
  const _CheckboxRow({
    required this.label,
    required this.value,
    required this.onChanged,
  });

  final String label;
  final bool value;
  final ValueChanged<bool> onChanged;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () => onChanged(!value),
      borderRadius: BorderRadius.circular(4),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Checkbox(
            value: value,
            onChanged: (value) => onChanged(value ?? false),
            visualDensity: VisualDensity.compact,
          ),
          Text(label),
          const SizedBox(width: 8),
        ],
      ),
    );
  }
}
