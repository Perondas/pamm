import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/rust/api/commands/init_from_remote.dart';
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
              Text(
                'Repository Path: ${widget.path}',
                style: TextStyle(fontWeight: FontWeight.bold),
              ),
              SizedBox(height: 16),
              if (widget.repo.description.isNotEmpty) ...[
                Text('Description: ${widget.repo.description}'),
              ],
              buildMoveButton(context),
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
}
