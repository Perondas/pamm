import 'package:flutter/material.dart';
import 'package:ui/src/models/stored_repo.dart';
import 'package:ui/src/pages/main_screen/repo_list/main.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  StoredRepo? _selectedRepo;

  void _onSelectRepo(StoredRepo repo) {
    setState(() {
      _selectedRepo = repo;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      //appBar: AppBar(title: Text("Pamm"), elevation: 1),
      body: Row(
        children: [
          RepoList(_onSelectRepo),
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
