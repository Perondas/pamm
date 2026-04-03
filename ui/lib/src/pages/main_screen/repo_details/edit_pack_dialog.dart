import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/optionals_list.dart';

import 'externals_list.dart';
import 'params.dart';

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
        height: 600,
        child: DefaultTabController(
          length: 3,
          child: Column(
            children: [
              const TabBar(
                tabs: [
                  Tab(text: 'Optionals'),
                  Tab(text: 'Externals'),
                  Tab(text: 'Launch Params'),
                ],
              ),
              Expanded(
                child: TabBarView(
                  children: [
                    OptionalsList(widget.repoPath, widget.packName),
                    ExternalsList(widget.repoPath, widget.packName),
                    ParamsForm(widget.repoPath, widget.packName),
                  ],
                ),
              ),
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
