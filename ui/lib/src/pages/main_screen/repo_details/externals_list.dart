import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:open_folder/open_folder.dart';
import 'package:pamm_ui/src/rust/api/commands/externals/load_externals.dart';
import 'package:pamm_ui/src/rust/api/commands/externals/save_externals.dart';

class ExternalsList extends StatefulWidget {
  const ExternalsList(this.repotPath, this.packName, {super.key});

  final String repotPath;
  final String packName;

  @override
  State<ExternalsList> createState() => _ExternalsListState();
}

class _ExternalsListState extends State<ExternalsList> {
  bool loading = true;
  List<FlutterExternalAddon> externals = [];

  @override
  void initState() {
    super.initState();
    _loadExternals();
  }

  Future<void> _loadExternals() async {
    setState(() => loading = true);
    try {
      final externals = await loadExternals(
        repotPath: widget.repotPath,
        packName: widget.packName,
      );
      externals.sort((a, b) {
        final nameA = a.name ?? a.path;
        final nameB = b.name ?? b.path;
        return nameA.compareTo(nameB);
      });
      if (!mounted) return;
      setState(() {
        this.externals = externals;
        loading = false;
      });
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to load externals: $e')));
    }
  }

  Future<void> _saveExternals() async {
    try {
      await saveExternals(
        repoPath: widget.repotPath,
        packName: widget.packName,
        externals: externals,
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to save externals: $e')));
      return;
    }
    _loadExternals();
  }

  Future<void> _openFolder(String path) async {
    try {
      await OpenFolder.openFolder(path);
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to open folder: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: ElevatedButton.icon(
              onPressed: () async {
                final selected = await FilePicker.getDirectoryPath();
                if (selected != null) {
                  var newExternal = FlutterExternalAddon(
                    name: null,
                    enabled: true,
                    path: selected,
                    foundInWorkshop: false,
                  );

                  externals.add(newExternal);
                  _saveExternals();
                }
              },
              label: Text('Add External Addon'),
              icon: Icon(Icons.add),
            ),
          ),
          if (loading) CircularProgressIndicator(),
          for (int i = 0; i < externals.length; i++)
            Row(
              children: [
                if (externals[i].foundInWorkshop) ...[
                  const Tooltip(
                    message: "Found in your Workshop",
                    child: Icon(Icons.info, color: Colors.blue),
                  ),
                ] else ...[
                  IconButton(
                    onPressed: () {
                      externals.removeAt(i);
                      _saveExternals();
                    },
                    icon: Icon(Icons.delete_forever, color: Colors.red),
                  ),
                ],

                Expanded(
                  child: SwitchListTile(
                    title: Text(
                      externals[i].name ??
                          externals[i].path.split(Platform.pathSeparator).last,
                    ),
                    subtitle: Align(
                      alignment: Alignment.centerLeft,
                      child: Tooltip(
                        message: 'Open folder',
                        child: MouseRegion(
                          cursor: SystemMouseCursors.click,
                          child: GestureDetector(
                            onTap: () => _openFolder(externals[i].path),
                            child: Text(
                              externals[i].path,
                              style: Theme.of(context).textTheme.labelSmall
                                  ?.copyWith(
                                    color: Theme.of(
                                      context,
                                    ).colorScheme.primary,
                                    decoration: TextDecoration.underline,
                                    decorationColor: Theme.of(
                                      context,
                                    ).colorScheme.primary,
                                  ),
                            ),
                          ),
                        ),
                      ),
                    ),
                    value: externals[i].enabled,
                    onChanged: (value) {
                      setState(() {
                        externals.setRange(i, i + 1, [
                          FlutterExternalAddon(
                            name: externals[i].name,
                            enabled: value,
                            foundInWorkshop: externals[i].foundInWorkshop,
                            path: externals[i].path,
                          ),
                        ]);
                      });
                      _saveExternals();
                    },
                  ),
                ),
              ],
            ),
        ],
      ),
    );
  }
}
