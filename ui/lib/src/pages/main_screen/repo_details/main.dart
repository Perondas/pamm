import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/edit_pack_dialog.dart';
import 'package:pamm_ui/src/pages/sync_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/quick_check.dart';

class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final RepoWithPath selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  @override
  Widget build(BuildContext context) {
    var repo = widget.selectedRepo.repo;
    var sortedPacks = repo.packs.toList();
    sortedPacks.sort();

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

  const PackListTile({
    required this.packName,
    required this.repoPath,
    super.key,
  });

  @override
  State<PackListTile> createState() => _PackListTileState();
}

class _PackListTileState extends State<PackListTile> {
  bool? _upToDate;

  @override
  void initState() {
    super.initState();
    _checkStatus();
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
    final String? imageUrl = null; // TODO: Implement image URL in PackConfig
    final Widget leadingWidget = imageUrl != null && imageUrl.isNotEmpty
        ? CircleAvatar(backgroundImage: NetworkImage(imageUrl))
        : CircleAvatar(
            child: Text(
              widget.packName.isNotEmpty
                  ? widget.packName[0].toUpperCase()
                  : '?',
            ),
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
                      SyncScreen(widget.packName, widget.repoPath),
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
