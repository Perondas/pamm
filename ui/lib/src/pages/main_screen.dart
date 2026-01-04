import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/dialogs/add_pack_dialog.dart';

class MainScreen extends StatelessWidget {
  const MainScreen({super.key});

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
                      onPressed: () async {
                        final result = await showDialog<RepoConfig?>(
                          context: context,
                          builder: (_) => AddPackDialog(),
                        );
                        if (result != null) {
                          if (!context.mounted) return;
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(content: Text('Added repo: ${result.name}')),
                          );
                        }
                      },
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
                  ...getAllRepos()
                ],
              ),
            ),
          ),
          Expanded(
            flex: 3,
            child: Container(
              color: Colors.white,
              child: Center(child: Text("Main Content Area")),
            ),
          ),
        ],
      ),
    );
  }
}

List<Widget> getAllRepos() {
  return [
    ListTile(
      leading: Icon(Icons.folder),
      title: Text("Sample Repo 1"),
      subtitle: Text("Last updated: Today"),
      onTap: () {},
    ),
    ListTile(
      leading: Icon(Icons.folder),
      title: Text("Sample Repo 2"),
      subtitle: Text("Last updated: Yesterday"),
      onTap: () {},
    ),
  ];
}
