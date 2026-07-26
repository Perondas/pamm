import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/main.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/main.dart';
import 'package:pamm_ui/src/pages/settings_screen/main.dart';
import 'package:pamm_ui/src/services/theme_service.dart';

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  RepoWithPath? _selectedRepo;

  void _onSelectRepo(RepoWithPath? repo) {
    themeService.applyRepo(repo?.repo);
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
                child: IconButton(
                  onPressed: () {
                    Navigator.of(context).push(
                      MaterialPageRoute(builder: (context) => SettingsScreen()),
                    );
                  },
                  icon: Icon(Icons.settings),
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
