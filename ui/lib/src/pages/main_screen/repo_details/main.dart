import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/add_local_pack_dialog.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/edit_pack_dialog.dart';
import 'package:pamm_ui/src/pages/sync_screen/main.dart';
import 'package:pamm_ui/src/pages/sync_single_pack_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/add_local_pack.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';
import 'package:pamm_ui/src/rust/api/commands/load_pack_display.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/quick_check.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/services/repo_state_store.dart';
import 'package:pamm_ui/src/services/settings_service.dart';
import 'package:pamm_ui/src/util/media.dart';
import 'package:pamm_ui/src/widgets/media_icon.dart';

class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final RepoStateManager selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.selectedRepo,
      builder: (BuildContext context, Widget? child) {
        var repo = widget.selectedRepo.repoState!.repo;
        var sortedPacks = repo.packs.toList();
        sortedPacks.sort();

        final banner = _buildBanner();

        return Padding(
          padding: const EdgeInsets.all(16.0),
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                //AppBar(title: Text(repo?.name ?? '')),
                Text(repo.name, style: Theme.of(context).textTheme.titleLarge),
                if (repo.description.isNotEmpty) ...[
                  const SizedBox(height: 8),
                  Text(repo.description),
                ],
                const SizedBox(height: 12),
                Text('Path:', style: TextStyle(fontWeight: FontWeight.bold)),
                SelectableText(widget.selectedRepo.repoPath),
                if (banner != null) ...[const SizedBox(height: 12), banner],
                const SizedBox(height: 12),
                Text('Packs:', style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                Flexible(
                  child: repo.packs.isEmpty
                      ? ListTile(
                          leading: Icon(Icons.info_outline),
                          title: Text('No packs found in this repository'),
                        )
                      : ListView.builder(
                          itemBuilder: (context, index) => PackListTile(
                            packName: sortedPacks[index],
                            repoPath: widget.selectedRepo.repoPath,
                            repoIconName: repo.customization?.icon,
                          ),
                          itemCount: repo.packs.length,
                          shrinkWrap: true,
                        ),
                ),
                if (settingsService.settings.mmSettings.mmModeEnabled) ...[
                  ...buildLocalPacksList(),
                ],
              ],
            ),
          ),
        );
      },
    );
  }

  /// The repo's banner, when it has one.
  Widget? _buildBanner() {
    final banner = mediaFile(
      widget.selectedRepo.repoPath,
      widget.selectedRepo.repoState!.repo.customization?.banner,
    );
    if (banner == null) return null;

    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: Image(
        image: MediaFileImage(banner),
        errorBuilder: (_, _, _) => const SizedBox.shrink(),
      ),
    );
  }

  List<Widget> buildLocalPacksList() {
    var localPacks = widget.selectedRepo.repoState!.settings.localPacks
        .toList();
    localPacks.sort();

    return [
      SizedBox(height: 12),
      Row(
        children: [
          Text('Local Packs:', style: TextStyle(fontWeight: FontWeight.bold)),
          const SizedBox(width: 8),
          Tooltip(
            message:
                'Local packs are packs that are treated like part of the repository but are stored locally on your machine.',
            child: Icon(Icons.info_outline, size: 16),
          ),
        ],
      ),
      const SizedBox(height: 8),
      ElevatedButton.icon(
        onPressed: _onAddLocalPack,
        icon: Icon(Icons.add),
        label: Text("Add local pack"),
        style: ElevatedButton.styleFrom(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadiusGeometry.all(Radius.circular(20)),
          ),
        ),
      ),
      const SizedBox(height: 8),
      Flexible(
        child: localPacks.isEmpty
            ? ListTile(
                leading: Icon(Icons.info_outline),
                title: Text('No local packs found in this repository'),
              )
            : ListView.builder(
                itemBuilder: (context, index) => PackListTile(
                  packName: localPacks[index],
                  repoPath: widget.selectedRepo.repoPath,
                  repoIconName:
                      widget.selectedRepo.repoState!.repo.customization?.icon,
                ),
                itemCount: localPacks.length,
                shrinkWrap: true,
              ),
      ),
    ];
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

class PackListTile extends StatefulWidget {
  final String packName;
  final String repoPath;
  final String? repoIconName;

  const PackListTile({
    required this.packName,
    required this.repoPath,
    this.repoIconName,
    super.key,
  });

  @override
  State<PackListTile> createState() => _PackListTileState();
}

class _PackListTileState extends State<PackListTile> {
  bool? _upToDate;
  PackDisplayInfo? _display;

  @override
  void initState() {
    super.initState();
    _checkStatus();
    _loadDisplay();
  }

  Future<void> _loadDisplay() async {
    try {
      final display = await loadPackDisplay(
        repoPath: widget.repoPath,
        packName: widget.packName,
      );
      if (mounted) {
        setState(() {
          _display = display;
        });
      }
    } catch (e) {
      // Media is cosmetic; fall back to the letter avatar.
    }
  }

  Future<void> _checkStatus() async {
    try {
      final upToDate = await quickCheck(
        packName: widget.packName,
        repoPath: widget.repoPath,
      );
      if (mounted) {
        setState(() {
          _upToDate = upToDate;
        });
      }
    } catch (e) {
      // Ignore errors for quick check
    }
  }

  @override
  Widget build(BuildContext context) {
    final Widget leadingWidget = MediaIcon(
      iconFile:
          mediaFile(widget.repoPath, _display?.icon) ??
          mediaFile(widget.repoPath, widget.repoIconName),
      fallback: widget.packName,
    );

    return ListTile(
      leading: leadingWidget,
      title: Text(widget.packName),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(
            onPressed: () async {
              try {
                await launch(
                  repoDir: widget.repoPath,
                  packName: widget.packName,
                  launchType: settingsService.settings.armaSettings.launchType,
                );
              } catch (e) {
                if (!context.mounted) return;
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text("Error launching pack: $e"),
                    backgroundColor: Colors.red,
                  ),
                );
              }
            },
            icon: Icon(Icons.play_arrow),
          ),
          IconButton(
            onPressed: () async {
              await Navigator.of(context).push(
                MaterialPageRoute(
                  builder: (context) =>
                      debugSettingsService.useLegacySinglePackSync
                      ? SyncSinglePackScreen(widget.packName, widget.repoPath)
                      : SyncScreen(widget.packName, widget.repoPath),
                ),
              );
              // Re-check status after returning from sync
              _checkStatus();
            },
            tooltip: _upToDate == false ? 'Updates available' : 'Sync pack',
            icon: Badge(
              isLabelVisible: _upToDate == false,
              child: Icon(Icons.download),
            ),
          ),
          IconButton(
            onPressed: () async {
              await showDialog(
                context: context,
                builder: (_) =>
                    EditPackDialog(widget.repoPath, widget.packName),
              );
            },
            icon: Icon(Icons.settings),
          ),
        ],
      ),
    );
  }
}
