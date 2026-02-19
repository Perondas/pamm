import 'package:flutter/material.dart';
import 'package:ui/src/models/repo_with_path.dart';
import 'package:ui/src/pages/main_screen/repo_list/edit_repo_dialog.dart';
import 'package:ui/src/pages/main_screen/repo_list/fix_repo_dialog.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/services/repo_path_store.dart';
import 'package:ui/src/services/repo_state_store.dart';
import 'package:ui/src/widgets/confirm_dialog.dart';

import 'add_repo_dialog.dart';

class RepoList extends StatefulWidget {
  const RepoList(this.selectedRepoChanged, {super.key});

  final ValueChanged<RepoWithPath?> selectedRepoChanged;

  @override
  State<RepoList> createState() => _RepoListState();
}

class _RepoListState extends State<RepoList> {
  List<RepoStateManager> _repos = [];
  RepoWithPath? _selectedRepo;

  @override
  void initState() {
    super.initState();
    _loadRepos();
  }

  Future<void> _loadRepos() async {
    final set = await RepoPathStore.getRepoPaths();
    if (!mounted) return;
    var selectedExists = set.contains(_selectedRepo?.path);
    var list = set.toList();
    list.sort();

    setState(() {
      if (!selectedExists) {
        _selectedRepo = null;
      }
      _repos = list.map((path) => RepoStateManager(path)).toList();
    });

    widget.selectedRepoChanged(_selectedRepo);
  }

  Future<void> _onAddRepo() async {
    final result = await showDialog<RepoWithPath?>(
      context: context,
      builder: (_) => AddRepoDialog(),
    );
    if (result != null) {
      // reload store to include newly added repo
      await _loadRepos();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: ElevatedButton.icon(
            onPressed: _onAddRepo,
            icon: Icon(Icons.add),
            label: Text("Add Repository"),
            style: ElevatedButton.styleFrom(
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadiusGeometry.all(Radius.circular(20)),
              ),
            ),
          ),
        ),
        Expanded(
          child: _repos.isEmpty
              ? ListTile(
                  leading: Icon(Icons.info_outline),
                  title: Text('No repositories added'),
                  subtitle: Text('Click "Add Repository" to add one'),
                )
              : ListView.builder(
                  itemBuilder: (context, index) =>
                      _buildRepoListTitle(_repos[index]),
                  itemCount: _repos.length,
                ),
        ),
      ],
    );
  }

  Widget _buildRepoListTitle(RepoStateManager repoStateManager) {
    return ListenableBuilder(
      listenable: repoStateManager,
      builder: (context, snapshot) {
        if (!repoStateManager.doneLoading) {
          return ListTile(
            leading: CircularProgressIndicator(),
            title: Text(repoStateManager.repoPath),
            subtitle: Text("Loading..."),
          );
        }

        if (repoStateManager.configLoadError != null) {
          return ListTile(
            leading: Icon(Icons.error, color: Colors.red),
            title: Text(repoStateManager.repoPath),
            subtitle: Text("Error loading repo"),
            onTap: () async {
              final path = await showDialog<String?>(
                context: context,
                builder: (_) => FixRepoDialog(repoStateManager.repoPath),
              );

              if (path != null) {
                await RepoPathStore.remove(repoStateManager.repoPath);
                await RepoPathStore.add(path);
                await _loadRepos();
              }
            },
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                IconButton(
                  onPressed: () async {
                    final confirm =
                        await showDialog<bool?>(
                          context: context,
                          builder: (_) => ConfirmDialog(
                            content:
                                'Are you sure you want to delete the repository at "${repoStateManager.repoPath}"? This will not delete the files on disk.',
                          ),
                        ) ??
                        false;

                    if (confirm) {
                      await RepoPathStore.remove(repoStateManager.repoPath);
                    }
                    _loadRepos();
                  },
                  icon: Icon(Icons.delete_forever),
                ),
                IconButton(
                  onPressed: () async {
                    if (context.mounted) {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text(repoStateManager.configLoadError!),
                          backgroundColor: Colors.red,
                        ),
                      );
                    }
                  },
                  icon: Icon(Icons.bug_report),
                ),
              ],
            ),
          );
        }

        final repo = repoStateManager.repoState!.repo;
        final path = repoStateManager.repoState!.path;

        return ListTile(
          leading: Icon(Icons.folder),
          title: Text(repo.name),
          subtitle: Text(path),
          selected: _selectedRepo != null && path == _selectedRepo!.path,
          selectedTileColor: Colors.grey.shade200,
          onTap: () {
            setState(() {
              _selectedRepo = repoStateManager.repoState!;
            });
            widget.selectedRepoChanged(repoStateManager.repoState!);
          },
          trailing: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (repoStateManager.configUpdateError != null) ...[
                IconButton(
                  onPressed: () async {
                    if (context.mounted) {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text(repoStateManager.configUpdateError!),
                          backgroundColor: Colors.red,
                        ),
                      );
                    }
                  },
                  icon: Icon(Icons.warning, color: Colors.red),
                ),
              ],
              IconButton(
                onPressed: () async {
                  await showDialog<RepoConfig?>(
                    context: context,
                    builder: (_) => EditRepoDialog(repoStateManager.repoState!),
                  );
                  await _loadRepos();
                },
                icon: Icon(Icons.more_vert),
              ),
            ],
          ),
        );
      },
    );
  }
}
