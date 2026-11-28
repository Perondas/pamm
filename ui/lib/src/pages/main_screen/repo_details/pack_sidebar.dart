import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_state.dart';
import 'package:pamm_ui/src/util/media.dart';
import 'package:pamm_ui/src/widgets/media_icon.dart';
import 'package:pamm_ui/src/widgets/update_badge.dart';

/// Left-hand list of the repo's packs (and local packs in MM mode).
class PackSidebar extends StatelessWidget {
  const PackSidebar({
    required this.packs,
    required this.localPacks,
    required this.showLocalPacks,
    required this.selected,
    required this.repoIconName,
    required this.onSelect,
    required this.onAddLocalPack,
    super.key,
  });

  final List<PackStateManager> packs;
  final List<PackStateManager> localPacks;
  final bool showLocalPacks;
  final PackStateManager? selected;
  final String? repoIconName;
  final ValueChanged<PackStateManager> onSelect;
  final VoidCallback onAddLocalPack;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return ColoredBox(
      color: colors.surfaceContainerLow,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _SectionHeader(title: 'Available Packs'),
          Expanded(
            child: ListView(
              children: [
                if (packs.isEmpty)
                  const ListTile(
                    leading: Icon(Icons.info_outline),
                    title: Text('No packs found in this repository'),
                  ),
                for (var i = 0; i < packs.length; i++)
                  _PackRow(
                    pack: packs[i],
                    striped: i.isOdd,
                    selected: identical(packs[i], selected),
                    repoIconName: repoIconName,
                    onTap: () => onSelect(packs[i]),
                  ),
                if (showLocalPacks) ...[
                  _SectionHeader(
                    title: 'Local Packs',
                    info:
                        'Local packs are packs that are treated like part of the repository but are stored locally on your machine.',
                    action: FilledButton.tonalIcon(
                      onPressed: onAddLocalPack,
                      icon: const Icon(Icons.add, size: 18),
                      label: const Text('Add local pack'),
                    ),
                  ),
                  if (localPacks.isEmpty)
                    const ListTile(
                      leading: Icon(Icons.info_outline),
                      title: Text('No local packs found in this repository'),
                    ),
                  for (var i = 0; i < localPacks.length; i++)
                    _PackRow(
                      pack: localPacks[i],
                      striped: i.isOdd,
                      selected: identical(localPacks[i], selected),
                      repoIconName: repoIconName,
                      onTap: () => onSelect(localPacks[i]),
                    ),
                ],
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  const _SectionHeader({required this.title, this.info, this.action});

  final String title;
  final String? info;
  final Widget? action;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.fromLTRB(24, 20, 16, 16),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: theme.dividerColor)),
      ),
      child: Row(
        children: [
          Text(
            title,
            style: theme.textTheme.headlineSmall?.copyWith(
              fontWeight: FontWeight.w600,
            ),
          ),
          if (info != null) ...[
            const SizedBox(width: 8),
            Tooltip(
              message: info!,
              child: const Icon(Icons.info_outline, size: 18),
            ),
          ],
          const Spacer(),
          ?action,
        ],
      ),
    );
  }
}

class _PackRow extends StatelessWidget {
  const _PackRow({
    required this.pack,
    required this.striped,
    required this.selected,
    required this.repoIconName,
    required this.onTap,
  });

  final PackStateManager pack;
  final bool striped;
  final bool selected;
  final String? repoIconName;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colors = theme.colorScheme;

    return ListenableBuilder(
      listenable: pack,
      builder: (context, _) {
        final info = pack.info;

        final background = selected
            ? colors.secondaryContainer
            : striped
            ? colors.surfaceContainer
            : colors.surfaceContainerLowest;

        return Material(
          color: background,
          child: InkWell(
            onTap: onTap,
            child: Container(
              height: 64,
              padding: const EdgeInsets.only(left: 16, right: 16),
              decoration: BoxDecoration(
                border: Border(
                  left: BorderSide(
                    color: selected ? colors.primary : Colors.transparent,
                    width: 4,
                  ),
                ),
              ),
              child: Row(
                children: [
                  UpdateBadge(
                    visible: pack.upToDate == false,
                    size: 22,
                    child: MediaIcon(
                      iconFile:
                          mediaFile(pack.repoPath, pack.display?.icon) ??
                          mediaFile(pack.repoPath, repoIconName),
                      fallback: pack.packName,
                    ),
                  ),
                  const SizedBox(width: 20),
                  Flexible(
                    child: Text(
                      pack.packName,
                      overflow: TextOverflow.ellipsis,
                      style: theme.textTheme.titleMedium?.copyWith(
                        color: selected ? colors.onSecondaryContainer : null,
                      ),
                    ),
                  ),
                  if (info?.enabledCdlc != null) ...[
                    const SizedBox(width: 8),
                    Tooltip(
                      message:
                          'CDLC: ${info!.enabledCdlc!.toString().split('.').last}',
                      child: const Icon(Icons.warning_amber, size: 16),
                    ),
                  ],
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}
