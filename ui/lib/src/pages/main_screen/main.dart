import 'package:flutter/material.dart';
import 'package:ui/src/models/stored_repo.dart';
import 'package:ui/src/pages/main_screen/repo_details/main.dart';
import 'package:ui/src/pages/main_screen/repo_list/main.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  StoredRepo? _selectedRepo;

  void _onSelectRepo(StoredRepo? repo) {
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
          Expanded(
            child: Container(
              decoration: BoxDecoration(
                border: Border(
                  right: BorderSide(color: Colors.grey.shade500, width: 2.5),
                ),
              ),
              child: RepoList(_onSelectRepo),
            ),
          ),
          Expanded(
            flex: 2,
            child: Container(
              child: _selectedRepo == null
                  ? Center(child: Text("Select a repository to view details"))
                  : RepoDetails(_selectedRepo!),
            ),
          ),
        ],
      ),
    );
  }
}
