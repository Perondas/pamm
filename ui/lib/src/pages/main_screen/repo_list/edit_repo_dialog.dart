import 'package:flutter/material.dart';
import 'package:ui/src/models/repo_with_path.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/services/repo_path_store.dart';
import 'package:ui/src/widgets/confirm_dialog.dart';

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
      title: Text('Edit ${widget.repo.name}'),
      content: SizedBox(
        width: 600,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Repository Path: ${widget.path}'),
            SizedBox(height: 16),
            if (widget.repo.description.isNotEmpty) ...[
              Text('Description: ${widget.repo.description}'),
            ],
            FilledButton(
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
            ),
          ],
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
}
