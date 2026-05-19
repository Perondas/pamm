import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/file_change.dart';

class DiffAddonTile extends StatelessWidget {
  const DiffAddonTile({
    super.key,
    required this.addonName,
    required this.changes,
  });

  final String addonName;
  final List<FileChange> changes;

  @override
  Widget build(BuildContext context) {
    final downloadSize = changes.fold<int>(
      0,
      (prev, fc) => prev + _changeToDlSize(fc.change),
    );

    return ExpansionTile(
      title: Text(addonName),
      subtitle: Row(
        children: [
          Text('${changes.length} changes'),
          const SizedBox(width: 16),
          Text('Download size: ${format(downloadSize)}'),
        ],
      ),
      children: changes.map((fileChange) {
        return ListTile(
          title: Text('File: ${fileChange.filePath}'),
          subtitle: switch (fileChange.change) {
            ChangeType_Created(:final size) => Text(
              'Download - Size: ${format(size.toInt())}',
            ),
            ChangeType_Deleted() => const Text('Delete'),
            ChangeType_Modified(:final dlSize, :final sizeChange) => Text(
              'Patch - Size Change: ${format(sizeChange)}, Download Size: ${format(dlSize.toInt())}',
            ),
          },
        );
      }).toList(),
    );
  }
}

int _changeToDlSize(ChangeType change) {
  return switch (change) {
    ChangeType_Created(:final size) => size.toInt(),
    ChangeType_Deleted() => 0,
    ChangeType_Modified(:final dlSize) => dlSize.toInt(),
  };
}
