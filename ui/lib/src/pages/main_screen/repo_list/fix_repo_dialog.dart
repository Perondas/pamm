import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:ui/src/rust/api/commands/load_repo.dart';

class FixRepoDialog extends StatefulWidget {
  const FixRepoDialog(this.repoPath, {super.key});

  final String repoPath;

  @override
  State<FixRepoDialog> createState() => _FixRepoDialogState();
}

class _FixRepoDialogState extends State<FixRepoDialog> {
  @override
  void initState() {
    super.initState();
    _targetDirController.text = widget.repoPath;
  }

  String? _error;
  bool _isLoading = false;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Point PAMM to new repo location'),
      content: SizedBox(
        width: 600,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Stepper(
              steps: [_buildTargetDirectoryStep()],
              currentStep: 0,
              onStepCancel: _isLoading
                  ? null
                  : () => Navigator.of(context).pop(),
              onStepContinue: _tryLoadRepo,
              controlsBuilder: (context, detail) {
                return Padding(
                  padding: const EdgeInsets.all(10.0),
                  child: Row(
                    children: [
                      FilledButton(
                        onPressed: detail.onStepContinue,
                        child: Text("Load Repo"),
                      ),
                      SizedBox(width: 10),
                      OutlinedButton(
                        onPressed: detail.onStepCancel,
                        child: Text("Cancel"),
                      ),
                    ],
                  ),
                );
              },
            ),
            if (_error != null) ...[
              Text(_error!, style: TextStyle(color: Colors.red)),
            ],
          ],
        ),
      ),
    );
  }

  Future<void> _tryLoadRepo() async {
    var path = _targetDirController.text;
    if (path.isEmpty) {
      setState(() => _error = "Please enter a target directory");
      return;
    }

    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      var repo = await loadRepo(repoPath: path);
      if (!mounted) return;
      Navigator.of(context).pop(path);
    } catch (e) {
      setState(() => _error = "Failed to load repo at $path: $e");
    } finally {
      setState(() => _isLoading = false);
    }
  }

  final TextEditingController _targetDirController = TextEditingController();

  Step _buildTargetDirectoryStep() {
    return Step(
      title: Text("Target Directory"),
      content: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _targetDirController,
              decoration: InputDecoration(
                labelText: 'Target directory',
                hintText: Platform.isWindows
                    ? r'C:\path\to\find\repo\at'
                    : r'/path/to/find/repo/at',
              ),
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: _isLoading ? null : _pickTargetDir,
            icon: Icon(Icons.folder_open),
            tooltip: 'Choose folder',
          ),
        ],
      ),
    );
  }

  // New: open a native folder picker and write the chosen path to the controller.
  Future<void> _pickTargetDir() async {
    try {
      final selected = await FilePicker.platform.getDirectoryPath();
      if (selected != null) {
        _targetDirController.text = selected;
        setState(() {});
      }
    } catch (e) {
      setState(() => _error = e.toString());
    }
  }
}
