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
          RepoList(_onSelectRepo),
          RepoDetails(_selectedRepo),
        ],
      ),
    );
  }
}
