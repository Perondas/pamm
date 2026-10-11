import 'dart:io';

import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/main.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/main.dart';
import 'package:pamm_ui/src/pages/settings_screen/main.dart';
import 'package:pamm_ui/src/services/repo_state_store.dart';
import 'package:pamm_ui/src/services/theme_service.dart';
import 'package:pamm_ui/src/widgets/proton_dialog.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  RepoStateManager? _selectedRepo;

  void _onSelectRepo(RepoStateManager? repo) {
    themeService.applyRepo(repo?.repoState?.repo);
    setState(() {
      _selectedRepo = repo;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          NavigationDrawer(
            children: [
              SizedBox(
                height: MediaQuery.of(context).size.height - 70,
                child: RepoList(_onSelectRepo),
              ),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Row(
                  children: [
                    IconButton(
                      tooltip: "Settings",
                      onPressed: () {
                        Navigator.of(context).push(
                          MaterialPageRoute(
                            builder: (context) => SettingsScreen(),
                          ),
                        );
                      },
                      icon: Icon(Icons.settings),
                    ),
                    if (Platform.isLinux)
                      IconButton(
                        tooltip: "Proton",
                        onPressed: () => showDialog<void>(
                          context: context,
                          builder: (_) => const ProtonDialog(),
                        ),
                        icon: const Icon(Icons.wine_bar),
                      ),
                  ],
                ),
              ),
            ],
          ),
          Expanded(
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
