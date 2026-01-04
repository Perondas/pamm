import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';

import 'package:ui/src/rust/api/commands/get_remote_repo_info.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/models/stored_repo.dart';
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

  @override
  void dispose() {
    _remoteController.dispose();
    _targetDirController.dispose();
    super.dispose();
  }

  Future<void> _fetchRepoInfo() async {
    final remote = _remoteController.text.trim();
    if (remote.isEmpty) {
      setState(() => _error = "Please enter a repository URL");
      return;
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
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _isLoading = false);
    }
  }

  Future<void> _initFromRemote() async {
    final remote = _remoteController.text.trim();
    final target = _targetDirController.text.trim();
    if (remote.isEmpty || target.isEmpty) {
      setState(() => _error = "Both remote URL and target directory are required");
      return;
    }

    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final result = await Future(() => initFromRemote(remote: remote, targetDir: target));
      // Show the returned repo config as confirmation.
      setState(() => _repoInfo = result);
      // Persist the repo config and chosen path to shared_preferences
      final stored = StoredRepo.fromRepoConfig(result, target);
      await ReposStore.add(stored);
      if (!mounted) return; // Avoid using context after async gap
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Initialized repo "${result.name}" at $target')),
      );
      Navigator.of(context).pop(result);
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _isLoading = false);
    }
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

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Add Repository from Remote'),
      content: SizedBox(
        width: 600,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _remoteController,
              decoration: InputDecoration(
                labelText: 'Repository URL',
                hintText: 'https://github.com/owner/repo.git',
              ),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                ElevatedButton.icon(
                  onPressed: _isLoading ? null : _fetchRepoInfo,
                  icon: Icon(Icons.cloud_download),
                  label: Text('Fetch Info'),
                ),
                const SizedBox(width: 12),
                if (_isLoading) const CircularProgressIndicator(),
              ],
            ),
            const SizedBox(height: 12),
            if (_error != null) ...[
              Text(_error!, style: TextStyle(color: Colors.red)),
              const SizedBox(height: 8),
            ],
            if (_repoInfo != null) ...[
              Align(
                alignment: Alignment.centerLeft,
                child: Text('Name: ${_repoInfo!.name}', style: TextStyle(fontWeight: FontWeight.bold)),
              ),
              Align(
                alignment: Alignment.centerLeft,
                child: Text('Description: ${_repoInfo!.description}'),
              ),
              const SizedBox(height: 8),
              Align(
                alignment: Alignment.centerLeft,
                child: Text('Packs:', style: TextStyle(fontWeight: FontWeight.bold)),
              ),
              SizedBox(
                height: 80,
                child: SingleChildScrollView(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: _repoInfo!.packs.map((p) => Text('- $p')).toList(),
                  ),
                ),
              ),
              const SizedBox(height: 12),
              Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _targetDirController,
                      decoration: InputDecoration(
                        labelText: 'Target directory',
                        hintText: r'C:\path\to\repos\my-repo',
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
            ]
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: _isLoading ? null : () => Navigator.of(context).pop(),
          child: Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: (_repoInfo != null && !_isLoading) ? _initFromRemote : null,
          child: Text('Initialize'),
        ),
      ],
    );
  }
}
