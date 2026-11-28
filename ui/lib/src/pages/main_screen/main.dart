import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/main.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/add_repo_dialog.dart';
import 'package:pamm_ui/src/pages/main_screen/top_bar.dart';
import 'package:pamm_ui/src/services/repo_path_store.dart';
import 'package:pamm_ui/src/services/repo_state_store.dart';
import 'package:pamm_ui/src/services/theme_service.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  List<RepoStateManager> _repos = [];
  RepoStateManager? _selectedRepo;

  @override
  void initState() {
    super.initState();
    _loadRepos();
  }

  void _onSelectRepo(RepoStateManager? repo) {
    themeService.applyRepo(repo?.repoState?.repo);
    setState(() {
      _selectedRepo = repo;
    });
  }

  Future<void> _loadRepos() async {
    final set = await RepoPathStore.getRepoPaths();
    if (!mounted) return;
    var selectedExists = set.contains(_selectedRepo?.repoPath);
    var list = set.toList();
    list.sort();

    setState(() {
      _repos = list.map((path) => RepoStateManager(path)).toList();
    });

    _onSelectRepo(selectedExists ? _selectedRepo : null);
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
    final selected = _selectedRepo;

    return Scaffold(
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          TopBar(
            repos: _repos,
            selectedRepo: selected,
            onSelectRepo: _onSelectRepo,
            onAddRepo: _onAddRepo,
            onReposChanged: _loadRepos,
          ),
          Expanded(
            child: selected == null
                ? Center(
                    child: _repos.isEmpty
                        ? const Column(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(Icons.info_outline, size: 32),
                              SizedBox(height: 8),
                              Text('No repositories added'),
                              Text(
                                'Use the repository menu in the top left to add one',
                              ),
                            ],
                          )
                        : const Text("Select a repository to view details"),
                  )
                : RepoDetails(selected, key: ValueKey(selected)),
          ),
        ],
      ),
    );
  }
}
