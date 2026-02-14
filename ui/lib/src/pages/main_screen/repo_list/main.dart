import 'package:flutter/material.dart';
import 'package:ui/src/pages/main_screen/repo_list/edit_repo_dialog.dart';
import 'package:ui/src/models/repo_with_path.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/services/repo_path_store.dart';
import 'add_repo_dialog.dart';

class RepoList extends StatefulWidget {
  const RepoList(this.selectedRepoChanged, {super.key});

  final ValueChanged<RepoWithPath?> selectedRepoChanged;

  @override
  State<RepoList> createState() => _RepoListState();
}

class _RepoListState extends State<RepoList> {
  List<String> _repoPaths = [];
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
      _repoPaths = list;
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
          child: _repoPaths.isEmpty
              ? ListTile(
                  leading: Icon(Icons.info_outline),
                  title: Text('No repositories added'),
                  subtitle: Text('Click "Add Repository" to add one'),
                )
              : ListView.builder(
                  itemBuilder: (context, index) =>
                      _buildRepoListTitle(_repoPaths[index]),
                  itemCount: _repoPaths.length,
                ),
        ),
      ],
    );
  }

  ListTile _buildRepoListTitle(RepoWithPath r) {
    return ListTile(
      leading: Icon(Icons.folder),
      title: Text(r.repo.name),
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
