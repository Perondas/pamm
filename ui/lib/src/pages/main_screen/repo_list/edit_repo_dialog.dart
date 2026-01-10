import 'package:flutter/material.dart';
import 'package:ui/src/models/stored_repo.dart';
import 'package:ui/src/services/repos_store.dart';
import 'package:ui/src/widgets/confirm_dialog.dart';

class EditRepoDialog extends StatefulWidget {
  const EditRepoDialog(this.repo, {super.key});

  final StoredRepo repo;

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
            Text('Repository Path: ${widget.repo.path}'),
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
                  await ReposStore.remove(widget.repo);
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
