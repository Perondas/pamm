import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/local_pack/add_local_pack.dart';

class AddLocalPackDialog extends StatefulWidget {
  const AddLocalPackDialog({super.key, required this.possibleParents});

  final List<String> possibleParents;

  @override
  State<AddLocalPackDialog> createState() => _AddLocalPackDialogState();
}

class _AddLocalPackDialogState extends State<AddLocalPackDialog> {
  late final List<String> sortedParents = widget.possibleParents.toList()
    ..sort();
  final Set<String> selectedParents = {};
  Cdlc? selectedCdlc;
  String? selectedName;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text("Add Local Pack"),
      content: SizedBox(
        width: 500,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextFormField(
              decoration: InputDecoration(
                hintText: "Name of the pack",
                label: Text("Name"),
              ),
              onChanged: (value) {
                selectedName = value;
              },
            ),
            const SizedBox(height: 16),
            Text("Parent Packs", style: Theme.of(context).textTheme.labelLarge),
            const SizedBox(height: 8),
            if (sortedParents.isEmpty)
              Text("No packs available")
            else
              ConstrainedBox(
                constraints: const BoxConstraints(maxHeight: 200),
                child: SingleChildScrollView(
                  child: Wrap(
                    spacing: 8,
                    runSpacing: 8,
                    children: sortedParents
                        .map(
                          (parent) => FilterChip(
                            label: Text(parent),
                            selected: selectedParents.contains(parent),
                            onSelected: (selected) {
                              setState(() {
                                if (selected) {
                                  selectedParents.add(parent);
                                } else {
                                  selectedParents.remove(parent);
                                }
                              });
                            },
                          ),
                        )
                        .toList(),
                  ),
                ),
              ),
            const SizedBox(height: 16),
            DropdownButtonFormField<Cdlc?>(
              decoration: InputDecoration(
                hintText: "Select a CDLC",
                label: Text("CDLC"),
              ),
              items:
                  Cdlc.values
                      .map(
                        (value) => DropdownMenuItem(
                          value: value,
                          child: Text(value.toString().split('.').last),
                        ),
                      )
                      .toList() +
                  [DropdownMenuItem(value: null, child: Text('None'))],
              onChanged: (value) {
                selectedCdlc = value;
              },
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            if (selectedName != null) {
              Navigator.of(context).pop(
                FlutterLocalPackConfig(
                  name: selectedName!,
                  // Listed order, so the saved config doesn't depend on click order.
                  parents: sortedParents
                      .where(selectedParents.contains)
                      .toList(),
                  enabledCdlc: selectedCdlc,
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
