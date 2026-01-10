import 'dart:io' show Platform;

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:ui/src/models/stored_repo.dart';
import 'package:ui/src/rust/api/commands/get_remote_repo_info.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/services/repos_store.dart';

class AddPackDialog extends StatefulWidget {
  const AddPackDialog({super.key});

  @override
  State<AddPackDialog> createState() => _AddPackDialogState();
}

class _AddPackDialogState extends State<AddPackDialog> {
  final TextEditingController _remoteController = TextEditingController();
  final TextEditingController _targetDirController = TextEditingController();

  RepoConfig? _repoInfo;
  String? _error;
  bool _isLoading = false;
  int _index = 0;

  @override
  void dispose() {
    _remoteController.dispose();
    _targetDirController.dispose();
    super.dispose();
  }

  Future<bool> _fetchRepoInfo() async {
    final remote = _remoteController.text.trim();
    if (remote.isEmpty) {
      setState(() => _error = "Please enter a repository URL");
      return false;
    }

    setState(() {
      _isLoading = true;
      _error = null;
      _repoInfo = null;
    });

    try {
      // The generated binding is synchronous. Wrap in a Future to avoid
      // blocking synchronous UI work in the button handler.
      final info = await Future(() => getRemoteRepoInfo(remote: remote));
      setState(() => _repoInfo = info);
      return true;
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _isLoading = false);
    }

    return false;
  }

  Future<bool> _initFromRemote() async {
    final remote = _remoteController.text.trim();
    final target = _targetDirController.text.trim();
    if (remote.isEmpty || target.isEmpty) {
      setState(
        () => _error = "Both remote URL and target directory are required",
      );
      return false;
    }

    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final result = await Future(
        () => initFromRemote(remote: remote, targetDir: target),
      );
      // Show the returned repo config as confirmation.
      setState(() => _repoInfo = result);
      // Persist the repo config and chosen path to shared_preferences
      final stored = StoredRepo.fromRepoConfig(result, target);
      await ReposStore.add(stored);
      if (!mounted) return false; // Avoid using context after async gap
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Initialized repo "${result.name}" at $target')),
      );
      Navigator.of(context).pop(result);
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _isLoading = false);
    }

    return false;
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

  void _tryProgressStepper() async {
    var nextIndex = _index;
    switch (_index) {
      case 0:
        if (await _fetchRepoInfo()) {
          nextIndex = 1;
        }
      case 1:
        nextIndex = 2;
      case 2:
        await _initFromRemote();
    }

    setState(() {
      _index = nextIndex;
    });
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Add Repository from Remote'),
      content: SizedBox(
        width: 600,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Stepper(
              steps: _buildSteps(),
              currentStep: _index,
              onStepCancel: _isLoading
                  ? null
                  : () => Navigator.of(context).pop(),
              onStepContinue: _tryProgressStepper,
              controlsBuilder: (context, detail) {
                return Padding(
                  padding: const EdgeInsets.all(10.0),
                  child: Row(
                    children: [
                      FilledButton(
                        onPressed: detail.onStepContinue,
                        child: Text(_index != 2 ? "Next" : "Finish"),
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

  List<Step> _buildSteps() {
    return [
      _buildUrlEnterStep(),
      _buildDetailsReviewStep(),
      _buildTargetDirectoryStep(),
    ];
  }

  Step _buildUrlEnterStep() {
    return Step(
      title: Text("URL"),
      content: TextField(
        controller: _remoteController,
        decoration: InputDecoration(
          labelText: 'Repository URL',
          hintText: 'https://github.com/owner/repo.git',
        ),
        onSubmitted: (_) {
          _tryProgressStepper();
        },
      ),
    );
  }

  Step _buildDetailsReviewStep() {
    return Step(
      title: Text("Repo Details"),
      content: Column(
        children: [
          Align(
            alignment: Alignment.centerLeft,
            child: Text(
              'Name: ${_repoInfo?.name}',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
          ),
          Align(
            alignment: Alignment.centerLeft,
            child: Text('Description: ${_repoInfo?.description}'),
          ),
          const SizedBox(height: 8),
          Align(
            alignment: Alignment.centerLeft,
            child: Text(
              'Packs:',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
          ),
          SizedBox(
            height: 80,
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children:
                    _repoInfo?.packs.map((p) => Text('- $p')).toList() ?? [],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Step _buildTargetDirectoryStep() {
    return Step(
      title: Text("Local Directory"),
      content: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _targetDirController,
              decoration: InputDecoration(
                labelText: 'Target directory',
                hintText: Platform.isWindows
                    ? r'C:\path\to\store\repo\at'
                    : r'/path/to/store/repo/at',
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
}
