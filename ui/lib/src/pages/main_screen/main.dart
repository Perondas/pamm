import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/pages/log_screen/main.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/main.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/main.dart';

export 'package:pamm_ui/src/pages/main_screen/main.dart' show WindowButtons;

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  RepoWithPath? _selectedRepo;

  void _onSelectRepo(RepoWithPath? repo) {
    setState(() {
      _selectedRepo = repo;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: WindowBorder(
        color: Theme.of(context).colorScheme.primary,
        child: Row(
          children: [
            NavigationDrawer(
              children: [
                WindowTitleBarBox(child: MoveWindow()),
                SizedBox(
                  height: MediaQuery.of(context).size.height - 90,
                  child: RepoList(_onSelectRepo),
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: IconButton(
                    onPressed: () {
                      Navigator.of(context).push(
                        MaterialPageRoute(builder: (context) => LogScreen()),
                      );
                    },
                    icon: Icon(Icons.bug_report),
                  ),
                ),
              ],
            ),
            Expanded(
              child: Column(
                children: [
                  WindowTitleBarBox(
                    child: Row(
                      children: [
                        Expanded(child: MoveWindow()),
                        const WindowButtons(),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Container(
                      child: _selectedRepo == null
                          ? const Center(child: Text("Select a repository to view details"))
                          : RepoDetails(_selectedRepo!),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

final buttonColors = WindowButtonColors(
    iconNormal: const Color(0xFF805306),
    mouseOver: const Color(0xFFF6A00C),
    mouseDown: const Color(0xFF805306),
    iconMouseOver: const Color(0xFF805306),
    iconMouseDown: const Color(0xFFFFD500));

final closeButtonColors = WindowButtonColors(
    mouseOver: const Color(0xFFD32F2F),
    mouseDown: const Color(0xFFB71C1C),
    iconNormal: const Color(0xFF805306),
    iconMouseOver: Colors.white);

class WindowButtons extends StatelessWidget {
  const WindowButtons({super.key});
  
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        MinimizeWindowButton(colors: buttonColors),
        MaximizeWindowButton(colors: buttonColors),
        CloseWindowButton(colors: closeButtonColors),
      ],
    );
  }
}
