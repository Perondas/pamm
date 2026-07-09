import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/file_change.dart';
import 'package:pamm_ui/src/widgets/diff_addon_tile.dart';

void main() {
  final changes = [
    FileChange(
      filePath: 'addons/created.pbo',
      change: ChangeType.created(size: BigInt.from(1000)),
    ),
    const FileChange(
      filePath: 'addons/deleted.pbo',
      change: ChangeType.deleted(),
    ),
    FileChange(
      filePath: 'addons/modified.pbo',
      change: ChangeType.modified(sizeChange: -200, dlSize: BigInt.from(500)),
    ),
  ];

  Future<void> pumpTile(WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DiffAddonTile(addonName: '@ace', changes: changes),
        ),
      ),
    );
  }

  testWidgets('shows addon name and change count', (tester) async {
    await pumpTile(tester);

    expect(find.text('@ace'), findsOneWidget);
    expect(find.text('3 changes'), findsOneWidget);
  });

  testWidgets('download size sums created and modified, not deleted',
      (tester) async {
    await pumpTile(tester);

    // 1000 (created) + 500 (modified dlSize); deletions cost nothing.
    expect(find.text('Download size: ${format(1500)}'), findsOneWidget);
  });

  testWidgets('expanding lists every file change', (tester) async {
    await pumpTile(tester);

    await tester.tap(find.text('@ace'));
    await tester.pumpAndSettle();

    expect(find.text('File: addons/created.pbo'), findsOneWidget);
    expect(find.text('Download - Size: ${format(1000)}'), findsOneWidget);
    expect(find.text('File: addons/deleted.pbo'), findsOneWidget);
    expect(find.text('Delete'), findsOneWidget);
    expect(find.text('File: addons/modified.pbo'), findsOneWidget);
    expect(
      find.text(
        'Patch - Size Change: ${format(-200)}, Download Size: ${format(500)}',
      ),
      findsOneWidget,
    );
  });
}
