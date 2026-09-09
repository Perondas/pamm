import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/local_pack/add_local_pack.dart';

class AddLocalPackDialog extends StatefulWidget {
  const AddLocalPackDialog({super.key, required this.possibleParents});

  final List<String?> possibleParents;

  @override
  State<AddLocalPackDialog> createState() => _AddLocalPackDialogState();
}

class _AddLocalPackDialogState extends State<AddLocalPackDialog> {
  String? selectedParent;
  String? selectedName;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text("Add Local Pack"),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          TextFormField(
            decoration: InputDecoration(hintText: "Name of the pack"),
            onChanged: (value) {
              selectedName = value;
            },
          ),
          DropdownButtonFormField<String?>(
            decoration: InputDecoration(hintText: "Select a parent pack"),
            items:
                widget.possibleParents
                    .map(
                      (parent) => DropdownMenuItem(
                        value: parent,
                        child: Text(parent ?? 'No Parent'),
                      ),
                    )
                    .toList() +
                [DropdownMenuItem(value: null, child: Text('None'))],
            onChanged: (value) {
              selectedParent = value;
            },
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () {
            if (selectedName != null) {
              Navigator.of(context).pop(
                FlutterLocalPackConfig(
                  name: selectedName!,
                  parent: selectedParent,
                ),
              );
            }
          },
          child: Text("Add"),
        ),
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: Text("Close"),
        ),
      ],
    );
  }
}
