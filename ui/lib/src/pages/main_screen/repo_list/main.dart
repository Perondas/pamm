import 'package:flutter/material.dart';
import 'package:ui/src/pages/main_screen/repo_list/edit_repo_dialog.dart';

import '../../../models/stored_repo.dart';
import '../../../rust/api/commands/init_from_remote.dart';
import '../../../services/repos_store.dart';
import 'add_repo_dialog.dart';

class RepoList extends StatefulWidget {
  const RepoList(this.selectedRepoChanged, {super.key});

  final ValueChanged<StoredRepo?> selectedRepoChanged;

  @override
  State<RepoList> createState() => _RepoListState();
}

class _RepoListState extends State<RepoList> {
  List<StoredRepo> _repos = [];
  StoredRepo? _selectedRepo;

  @override
  void initState() {
    super.initState();
    _loadRepos();
  }

  Future<void> _loadRepos() async {
    final list = await ReposStore.load();
    if (!mounted) return;
    var selectedExists = list.any((r) => r.path == _selectedRepo?.path);
    setState(() {
      if (!selectedExists) {
        _selectedRepo = null;
      }
      _repos = list;
    });

    widget.selectedRepoChanged(_selectedRepo);
  }

  Future<void> _onAddRepo() async {
    final result = await showDialog<RepoConfig?>(
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

  ListTile _buildRepoListTitle(StoredRepo r) {
    return ListTile(
      leading: Icon(Icons.folder),
      title: Text(r.name),
      subtitle: Text(r.path),
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
}
