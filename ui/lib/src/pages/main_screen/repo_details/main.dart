import 'package:flutter/material.dart';
import 'package:ui/src/models/repo_with_path.dart';
import 'package:ui/src/pages/main_screen/repo_details/edit_pack_dialog.dart';
import 'package:ui/src/pages/sync_screen/main.dart';
import 'package:ui/src/rust/api/commands/launch.dart';

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
                      itemBuilder: (context, index) => _buildPackListTitle(
                        context,
                        sortedPacks[index],
                        widget.selectedRepo.path,
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

ListTile _buildPackListTitle(
  BuildContext context,
  String packName,
  String repoPath,
) {
  return ListTile(
    leading: Icon(Icons.videogame_asset),
    title: Text(packName),
    trailing: Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        IconButton(
          onPressed: () async {
            await launch(repoDir: repoPath, packName: packName);
          },
          icon: Icon(Icons.play_arrow),
        ),
        IconButton(
          onPressed: () async {
            Navigator.of(context).push(
              MaterialPageRoute(
                builder: (context) => SyncScreen(packName, repoPath),
              ),
            );
          },
          icon: Icon(Icons.download),
        ),
        IconButton(
          onPressed: () async {
            await showDialog(
              context: context,
              builder: (_) => EditPackDialog(repoPath, packName),
            );
          },
          icon: Icon(Icons.settings),
        ),
      ],
    ),
  );
}
