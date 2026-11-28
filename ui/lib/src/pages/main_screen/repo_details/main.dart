import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/add_local_pack_dialog.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_details.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_sidebar.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_state.dart';
import 'package:pamm_ui/src/rust/api/commands/local_pack/add_local_pack.dart';
import 'package:pamm_ui/src/services/repo_state_store.dart';
import 'package:pamm_ui/src/services/settings_service.dart';

/// The body of the main screen for a selected repo: the pack list on the
/// left and the selected pack's details on the right.
class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final RepoStateManager selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  final Map<String, PackStateManager> _packs = {};
  String? _selectedKey;

  static String _key(String packName, bool isLocal) =>
      '${isLocal ? 'local' : 'repo'}:$packName';

  PackStateManager _pack(String packName, bool isLocal) {
    return _packs.putIfAbsent(
      _key(packName, isLocal),
      () => PackStateManager(
        packName: packName,
        repoPath: widget.selectedRepo.repoPath,
        isLocal: isLocal,
        onError: _showError,
      ),
    );
  }

  void _showError(String message) {
    if (!mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  void dispose() {
    for (final pack in _packs.values) {
      pack.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: Listenable.merge([widget.selectedRepo, settingsService]),
      builder: (BuildContext context, Widget? child) {
        final repoState = widget.selectedRepo.repoState;
        if (repoState == null) {
          return Center(
            child: Text(
              widget.selectedRepo.configLoadError ?? 'Failed to load repo',
            ),
          );
        }

        final mmMode = settingsService.settings.mmSettings.mmModeEnabled;

        final sortedPacks = repoState.repo.packs.toList()..sort();
        final packs = [for (final name in sortedPacks) _pack(name, false)];

        final localPacks = <PackStateManager>[];
        if (mmMode) {
          final names = repoState.settings.localPacks.toList()..sort();
          localPacks.addAll([for (final name in names) _pack(name, true)]);
        }

        // Keep the selection valid; default to the first pack.
        final visible = [...packs, ...localPacks];
        PackStateManager? selected = _packs[_selectedKey];
        if (selected == null || !visible.contains(selected)) {
          selected = visible.isEmpty ? null : visible.first;
          _selectedKey = selected == null
              ? null
              : _key(selected.packName, selected.isLocal);
        }

        final iconName = repoState.repo.customization?.icon;

        return Row(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            SizedBox(
              width: 380,
              child: PackSidebar(
                packs: packs,
                localPacks: localPacks,
                showLocalPacks: mmMode,
                selected: selected,
                repoIconName: iconName,
                onSelect: (pack) => setState(() {
                  _selectedKey = _key(pack.packName, pack.isLocal);
                }),
                onAddLocalPack: _onAddLocalPack,
              ),
            ),
            const VerticalDivider(width: 1, thickness: 1),
            Expanded(
              child: PackDetails(
                repoState: repoState,
                pack: selected,
                onLocalPackDeleted: () {
                  widget.selectedRepo.reLoad();
                },
              ),
            ),
          ],
        );
      },
    );
  }

  Future<void> _onAddLocalPack() async {
    final result = await showDialog<FlutterLocalPackConfig?>(
      context: context,
      builder: (_) => AddLocalPackDialog(
        possibleParents:
            widget.selectedRepo.repoState!.repo.packs.toList() +
            widget.selectedRepo.repoState!.settings.localPacks.toList(),
      ),
    );
    if (result != null) {
      await addLocalPack(
        repoPath: widget.selectedRepo.repoPath,
        config: result,
      );
      setState(() {
        widget.selectedRepo.reLoad();
      });
    }
  }
}
