import 'package:flutter/material.dart';
import 'package:ui/src/dialogs/add_pack_dialog.dart';
import 'package:ui/src/models/stored_repo.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/services/repos_store.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
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

  void _onSelectRepo(StoredRepo repo) {
    setState(() {
      _selectedRepo = repo;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Pamm"), elevation: 1),
      body: Row(
        children: [
          Expanded(
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
                          borderRadius: BorderRadiusGeometry.all(
                            Radius.circular(20),
                          ),
                        ),
                      ),
                    ),
                  ),
                  Expanded(
                    child: ListView(
                      children: _repos.isEmpty
                          ? [
                              ListTile(
                                leading: Icon(Icons.info_outline),
                                title: Text('No repositories added'),
                                subtitle: Text(
                                  'Click "Add Repository" to add one',
                                ),
                              ),
                            ]
                          : _repos
                                .map(
                                  (r) => ListTile(
                                    leading: Icon(Icons.folder),
                                    title: Text(r.name),
                                    subtitle: Text(r.path),
                                    selected:
                                        _selectedRepo != null &&
                                        r.path == _selectedRepo!.path,
                                    selectedTileColor: Colors.grey.shade200,
                                    onTap: () => _onSelectRepo(r),
                                  ),
                                )
                                .toList(),
                    ),
                  ),
                ],
              ),
            ),
          ),
          Expanded(
            flex: 3,
            child: Container(
              color: Colors.white,
              child: _selectedRepo == null
                  ? Center(child: Text("Select a repository to view details"))
                  : Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: SingleChildScrollView(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              _selectedRepo!.name,
                              style: Theme.of(context).textTheme.titleLarge,
                            ),
                            const SizedBox(height: 8),
                            Text(_selectedRepo!.description),
                            const SizedBox(height: 12),
                            Text(
                              'Path:',
                              style: TextStyle(fontWeight: FontWeight.bold),
                            ),
                            Text(_selectedRepo!.path),
                            const SizedBox(height: 12),
                            Text(
                              'Packs:',
                              style: TextStyle(fontWeight: FontWeight.bold),
                            ),
                            const SizedBox(height: 8),
                            if (_selectedRepo!.packs.isEmpty)
                              Text('No packs available')
                            else
                              ..._selectedRepo!.packs.map(
                                (p) => Padding(
                                  padding: const EdgeInsets.symmetric(
                                    vertical: 2.0,
                                  ),
                                  child: Text('- $p'),
                                ),
                              ),
                          ],
                        ),
                      ),
                    ),
            ),
          ),
        ],
      ),
    );
  }
}
