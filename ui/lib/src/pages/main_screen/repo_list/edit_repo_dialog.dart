import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/rust/api/commands/init_from_remote.dart';
import 'package:pamm_ui/src/rust/api/commands/user_repo_settings/load_settings.dart';
import 'package:pamm_ui/src/rust/api/commands/user_repo_settings/save_settings.dart';
import 'package:pamm_ui/src/rust/api/commands/user_repo_settings.dart';
import 'package:pamm_ui/src/services/repo_path_store.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';

class EditRepoDialog extends StatefulWidget {
  const EditRepoDialog(this.selectedRepo, {super.key});

  final RepoWithPath selectedRepo;

  String get path => selectedRepo.path;

  RepoConfig get repo => selectedRepo.repo;

  @override
  State<EditRepoDialog> createState() => _EditRepoDialogState();
}

class _EditRepoDialogState extends State<EditRepoDialog> {
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(
        'Edit ${widget.repo.name}',
        style: Theme.of(context).textTheme.headlineLarge,
      ),
      content: SizedBox(
        width: 600,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SelectableText(
                'Repository Path: ${widget.path}',
                style: TextStyle(fontWeight: FontWeight.bold),
              ),
              SizedBox(height: 16),
              if (widget.repo.description.isNotEmpty) ...[
                Text('Description: ${widget.repo.description}'),
              ],
              buildMoveButton(context),
              SizedBox(height: 16),
              buildRemoteEdit(context),
              SizedBox(height: 16),
              buildDeleteButton(context),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: Text('Close'),
        ),
      ],
    );
  }

  Widget buildDeleteButton(BuildContext context) {
    return FilledButton(
      onPressed: () async {
        final confirm =
            await showDialog<bool?>(
              context: context,
              builder: (_) => ConfirmDialog(
                content:
                    'Are you sure you want to delete the repository "${widget.repo.name}"? This will not delete the files on disk.',
              ),
            ) ??
            false;

        if (confirm) {
          await RepoPathStore.remove(widget.path);
          if (!mounted) return;
          Navigator.of(context).pop();
        }
      },
      style: ButtonStyle(
        backgroundColor: WidgetStateProperty.all(Colors.redAccent),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.delete_forever),
          SizedBox(width: 8),
          Text("Delete ${widget.repo.name}"),
        ],
      ),
    );
  }

  Widget buildMoveButton(BuildContext context) {
    return FilledButton(
      onPressed: () async {
        final confirm =
            await showDialog<bool?>(
              context: context,
              builder: (_) => ConfirmDialog(
                content:
                    'Are you sure you want to move the repository "${widget.repo.name}"?',
              ),
            ) ??
            false;

        if (!confirm) return;

        final selected = await FilePicker.getDirectoryPath(
          dialogTitle: "Select destination directory",
        );

        if (selected == null) return;

        final target = selected + Platform.pathSeparator + widget.repo.name;

        // Move the folder
        await Directory(widget.path).rename(target);

        await RepoPathStore.remove(widget.path);
        await RepoPathStore.add(target);
        if (!mounted) return;
        Navigator.of(context).pop();
      },
      style: ButtonStyle(
        backgroundColor: WidgetStateProperty.all(Colors.orangeAccent),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.move_up),
          SizedBox(width: 8),
          Text("Move ${widget.repo.name}"),
        ],
      ),
    );
  }

  Widget buildRemoteEdit(BuildContext context) {
    return FutureBuilder(
      future: loadSettings(repotPath: widget.path),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return CircularProgressIndicator();
        }

        final current = snapshot.data?.remote ?? '';

        return _RemoteEditor(
          initialUrl: current,
          repoPath: widget.path,
        );
      },
    );
  }

}

class _RemoteEditor extends StatefulWidget {
  const _RemoteEditor({required this.initialUrl, required this.repoPath});

  final String initialUrl;
  final String repoPath;

  @override
  State<_RemoteEditor> createState() => _RemoteEditorState();
}

class _RemoteEditorState extends State<_RemoteEditor> {
  late TextEditingController _controller;
  late String _initial;
  bool _editing = false;
  bool _isValid = true;
  bool _isSaving = false;
  String? _error;
  bool _saveFailed = false;

  @override
  void initState() {
    super.initState();
    _initial = widget.initialUrl;
    _controller = TextEditingController(text: _initial);
    _controller.addListener(_onChanged);
  }

  @override
  void dispose() {
    _controller.removeListener(_onChanged);
    _controller.dispose();
    super.dispose();
  }

  void _onChanged() {
    final text = _controller.text.trim();
    final valid = _validateUrl(text);
    setState(() {
      _isValid = valid;
      // clear error when user types
      _error = null;
      _saveFailed = false;
    });
  }

  bool _validateUrl(String url) {
    if (url.isEmpty) return false;
    final uri = Uri.tryParse(url);
    if (uri == null) return false;
    if (!(uri.hasScheme && (uri.scheme == 'http' || uri.scheme == 'https'))) return false;
    if (uri.host.isEmpty) return false;
    return true;
  }

  Future<void> _save() async {
    final newUrl = _controller.text.trim();
    if (!_validateUrl(newUrl)) {
      setState(() {
        _error = 'URL is not valid';
      });
      return;
    }

    if (newUrl == _initial) {
      // nothing changed
      setState(() {
        _editing = false;
      });
      return;
    }

    setState(() {
      _isSaving = true;
      _error = null;
    });

    try {
      await saveSettings(repoPath: widget.repoPath, setting: FlutterRepoUserSettings(remote: newUrl));
      setState(() {
        _initial = newUrl;
        _editing = false;
        _saveFailed = false;
      });
    } catch (e) {
      setState(() {
        _error = 'Failed to save: ${e.toString()}';
        _saveFailed = true;
      });
    } finally {
      if (mounted) {
        setState(() {
          _isSaving = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final changed = _controller.text.trim() != _initial;
    final canSave = !_isSaving && _isValid && changed && !_saveFailed;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Remote URL:',
          style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
        ),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: _controller,
                readOnly: !_editing || _isSaving,
                style: TextStyle(fontSize: 16),
                decoration: InputDecoration(
                  hintText: 'Not set',
                  hintStyle: TextStyle(fontSize: 16, color: Colors.grey),
                  errorText: _isValid ? null : 'Invalid URL',
                ),
              ),
            ),
            SizedBox(width: 8),
            if (!_editing) ...[
              IconButton(
                icon: Icon(Icons.edit),
                onPressed: () {
                  setState(() {
                    _editing = true;
                    _error = null;
                  });
                },
              ),
            ] else ...[
              IconButton(
                icon: _isSaving ? SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2)) : Icon(Icons.check),
                onPressed: canSave ? _save : null,
                tooltip: 'Save',
              ),
              IconButton(
                icon: Icon(Icons.close),
                onPressed: _isSaving
                    ? null
                    : () {
                        setState(() {
                          _controller.text = _initial;
                          _editing = false;
                          _error = null;
                        });
                      },
                tooltip: 'Cancel',
              ),
            ],
          ],
        ),
        if (_error != null) ...[
          SizedBox(height: 8),
          Text(_error!, style: TextStyle(color: Colors.redAccent)),
        ],
      ],
    );
  }
}
