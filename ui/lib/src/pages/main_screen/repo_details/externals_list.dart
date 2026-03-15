import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/externals/load_externals.dart';
import 'package:ui/src/rust/api/commands/externals/save_externals.dart';

class ExternalsList extends StatefulWidget {
  const ExternalsList(this.repotPath, this.packName, {super.key});

  final String repotPath;
  final String packName;

  @override
  State<ExternalsList> createState() => _ExternalsListState();
}

class _ExternalsListState extends State<ExternalsList> {
  bool loading = true;
  List<ExternalAddon> externals = [];

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

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          Text("Externals", style: Theme.of(context).textTheme.headlineSmall),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: ElevatedButton.icon(
              onPressed: () async {
                final selected = await FilePicker.platform.getDirectoryPath();
                if (selected != null) {
                  var newExternal = ExternalAddon(
                    name: null,
                    enabled: true,
                    path: selected,
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
            SwitchListTile(
              title: Text(
                externals[i].name ??
                    externals[i].path.split(Platform.pathSeparator).last,
              ),
              subtitle: Text(
                externals[i].path,
                style: Theme.of(context).textTheme.labelSmall,
              ),
              value: externals[i].enabled,
              onChanged: (value) {
                setState(() {
                  externals.setRange(i, i + 1, [
                    ExternalAddon(
                      name: externals[i].name,
                      enabled: value,
                      path: externals[i].path,
                    ),
                  ]);
                });
                _saveExternals();
              },
            ),
        ],
      ),
    );
  }
}
