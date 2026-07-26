import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/edit_pack_dialog.dart';
import 'package:pamm_ui/src/pages/sync_screen/main.dart';
import 'package:pamm_ui/src/pages/sync_single_pack_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';
import 'package:pamm_ui/src/rust/api/commands/load_pack_display.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/quick_check.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/util/media.dart';

class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final RepoWithPath selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  String? _selectedPack;
  PackDisplayInfo? _selectedPackDisplay;

  @override
  void didUpdateWidget(covariant RepoDetails oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.selectedRepo.path != widget.selectedRepo.path) {
      _selectedPack = null;
      _selectedPackDisplay = null;
    }
  }

  void _onSelectPack(String packName, PackDisplayInfo? display) {
    setState(() {
      if (_selectedPack == packName) {
        // Tapping the selected pack again deselects it.
        _selectedPack = null;
        _selectedPackDisplay = null;
      } else {
        _selectedPack = packName;
        _selectedPackDisplay = display;
      }
    });
  }

  /// Banner of the selected pack, falling back to the repo banner.
  Widget? _buildBanner() {
    if (_selectedPack == null) return null;

    final repoPath = widget.selectedRepo.path;
    final banner =
        mediaFile(repoPath, _selectedPackDisplay?.banner) ??
        mediaFile(repoPath, widget.selectedRepo.repo.customization?.banner);
    if (banner == null) return null;

    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: SizedBox(
        width: double.infinity,
        height: 180,
        child: Image.file(
          banner,
          fit: BoxFit.cover,
          errorBuilder: (_, _, _) => const SizedBox.shrink(),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    var repo = widget.selectedRepo.repo;
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
            const SizedBox(height: 8),
            Text(repo.description),
            const SizedBox(height: 12),
            Text('Path:', style: TextStyle(fontWeight: FontWeight.bold)),
            Text(widget.selectedRepo.path),
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
                        repoPath: widget.selectedRepo.path,
                        repoIconName: repo.customization?.icon,
                        selected: _selectedPack == sortedPacks[index],
                        onSelected: _onSelectPack,
                      ),
                      itemCount: repo.packs.length,
                      shrinkWrap: true,
                    ),
            ),
          ],
        ),
      ),
    );
  }
}

class PackListTile extends StatefulWidget {
  final String packName;
  final String repoPath;
  final String? repoIconName;
  final bool selected;
  final void Function(String packName, PackDisplayInfo? display)? onSelected;

  const PackListTile({
    required this.packName,
    required this.repoPath,
    this.repoIconName,
    this.selected = false,
    this.onSelected,
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
    final iconFile =
        mediaFile(widget.repoPath, _display?.icon) ??
        mediaFile(widget.repoPath, widget.repoIconName);
    final Widget leadingWidget = CircleAvatar(
      backgroundImage: iconFile != null ? FileImage(iconFile) : null,
      onBackgroundImageError: iconFile != null ? (_, _) {} : null,
      child: iconFile == null
          ? Text(
              widget.packName.isNotEmpty
                  ? widget.packName[0].toUpperCase()
                  : '?',
            )
          : null,
    );

    return ListTile(
      leading: leadingWidget,
      title: Text(widget.packName),
      selected: widget.selected,
      selectedTileColor: Theme.of(context).colorScheme.secondaryContainer,
      onTap: widget.onSelected == null
          ? null
          : () => widget.onSelected!(widget.packName, _display),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(
            onPressed: () async {
              try {
                await launch(
                  repoDir: widget.repoPath,
                  packName: widget.packName,
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
                          ? SyncSinglePackScreen(
                              widget.packName,
                              widget.repoPath,
                            )
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
