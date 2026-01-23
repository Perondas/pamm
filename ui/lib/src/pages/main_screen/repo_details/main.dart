import 'package:flutter/material.dart';
import 'package:ui/src/pages/sync_screen/main.dart';

import '../../../models/stored_repo.dart';

class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final StoredRepo selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            //AppBar(title: Text(widget.selectedRepo?.name ?? '')),
            Text(
              widget.selectedRepo.name,
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            Text(widget.selectedRepo.description),
            const SizedBox(height: 12),
            Text('Path:', style: TextStyle(fontWeight: FontWeight.bold)),
            Text(widget.selectedRepo.path),
            const SizedBox(height: 12),
            Text('Packs:', style: TextStyle(fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            Flexible(
              child: widget.selectedRepo.packs.isEmpty
                  ? ListTile(
                      leading: Icon(Icons.info_outline),
                      title: Text('No packs found in this repository'),
                    )
                  : ListView.builder(
                      itemBuilder: (context, index) => _buildPackListTitle(
                        context,
                        widget.selectedRepo.packs[index],
                        widget.selectedRepo.path + '\\' + widget.selectedRepo.name,
                      ),
                      itemCount: widget.selectedRepo.packs.length,
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
        IconButton(onPressed: () async {}, icon: Icon(Icons.play_arrow)),
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
      ],
    ),
  );
}
