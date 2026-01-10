import 'package:flutter/material.dart';

import '../../../models/stored_repo.dart';
import '../../../rust/api/commands/init_from_remote.dart';
import '../../../services/repos_store.dart';
import 'add_repo_dialog.dart';

class RepoList extends StatefulWidget {
  const RepoList(this.selectedRepoChanged, {super.key});

  final ValueChanged<StoredRepo> selectedRepoChanged;

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
    setState(() {
      _repos = list;
      // preserve selection if possible, otherwise select first
      if (_selectedRepo != null) {
        try {
          _selectedRepo = list.firstWhere((r) => r.path == _selectedRepo!.path);
        } catch (e) {
          _selectedRepo = list.isNotEmpty ? list.first : null;
        }
      } else {
        _selectedRepo = list.isNotEmpty ? list.first : null;
      }
    });
  }

  Future<void> _onAddRepo() async {
    final result = await showDialog<RepoConfig?>(
      context: context,
      builder: (_) => AddPackDialog(),
    );
    if (result != null) {
      // reload store to include newly added repo
      await _loadRepos();
      if (!context.mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Added repo: ${result.name}')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        decoration: BoxDecoration(
          border: Border(
            right: BorderSide(color: Colors.grey.shade500, width: 2.5),
          ),
        ),
        child: Column(
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
        ),
      ),
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
    );
  }
}
