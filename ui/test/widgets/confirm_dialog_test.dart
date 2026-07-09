import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';

void main() {
  Future<void> pumpDialog(
    WidgetTester tester, {
    String? title,
    String? content,
    void Function(bool?)? onResult,
  }) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Builder(
          builder: (context) => TextButton(
            onPressed: () async {
              final result = await showDialog<bool>(
                context: context,
                builder: (context) =>
                    ConfirmDialog(title: title, content: content),
              );
              onResult?.call(result);
            },
            child: const Text('open'),
          ),
        ),
      ),
    );
    await tester.tap(find.text('open'));
    await tester.pumpAndSettle();
  }

  testWidgets('shows title and content', (tester) async {
    await pumpDialog(tester, title: 'Delete repo?', content: 'This is final.');

    expect(find.text('Delete repo?'), findsOneWidget);
    expect(find.text('This is final.'), findsOneWidget);
  });

  testWidgets('tapping Yes pops with true', (tester) async {
    bool? result;
    await pumpDialog(tester, onResult: (r) => result = r);

    await tester.tap(find.text('Yes'));
    await tester.pumpAndSettle();

    expect(result, isTrue);
    expect(find.byType(ConfirmDialog), findsNothing);
  });

  testWidgets('tapping No pops with false', (tester) async {
    bool? result;
    await pumpDialog(tester, onResult: (r) => result = r);

    await tester.tap(find.text('No'));
    await tester.pumpAndSettle();

    expect(result, isFalse);
    expect(find.byType(ConfirmDialog), findsNothing);
  });
}
