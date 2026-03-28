import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/optionals_list.dart';

import 'externals_list.dart';

class EditPackDialog extends StatefulWidget {
  const EditPackDialog(this.repoPath, this.packName, {super.key});

  final String repoPath;
  final String packName;

  @override
  State<EditPackDialog> createState() => _EditPackDialogState();
}

class _EditPackDialogState extends State<EditPackDialog> {
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(
        'Edit ${widget.packName}',
        style: Theme.of(context).textTheme.headlineLarge,
      ),
      content: SizedBox(
        width: 600,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              OptionalsList(widget.repoPath, widget.packName),
              ExternalsList(widget.repoPath, widget.packName),
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
}
