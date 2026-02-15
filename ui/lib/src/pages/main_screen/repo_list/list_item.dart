import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/rust/api/commands/load_repo.dart';

class RepoListItem extends StatefulWidget {
  const RepoListItem(this.repoPath, {super.key});

  final String repoPath;

  @override
  State<RepoListItem> createState() => _RepoListItemState();
}

class _RepoListItemState extends State<RepoListItem> {
  late final Future<RepoConfig> _loadedRepoFuture = loadRepo(
    repoPath: widget.repoPath,
  );

  selected

  @override
  void initState() {
    super.initState();

    _loadedRepoFuture.then((_) => syncConfig());
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _loadedRepoFuture,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return ListTile(
            leading: CircularProgressIndicator(),
            title: Text(widget.repoPath),
            subtitle: Text("Loading..."),
          );
        } else if (snapshot.hasError) {
          return ListTile(
            leading: Icon(Icons.error, color: Colors.red),
            title: Text(widget.repoPath),
            subtitle: Text("Error loading repo"),
          );
        } else {
          final repo = snapshot.data!;
          return ListTile(
            leading: Icon(Icons.folder),
            title: Text(repo.name),
            subtitle: Text(widget.repoPath),
            selected: _selectedRepo != null && r.path == _selectedRepo!.path,
            selectedTileColor: Colors.grey.shade200,
            onTap: () {
              setState(() {
                _selectedRepo = r;
              });
              widget.selectedRepoChanged(r);
            },
            trailing: IconButton(
              onPressed: () async {
                await showDialog<RepoConfig?>(
                  context: context,
                  builder: (_) => EditRepoDialog(r),
                );
                await _loadRepos();
              },
              icon: Icon(Icons.more_vert),
            ),
          );
        }
      },
    );
  }

  void syncConfig() {
    // TODO
  }
}
