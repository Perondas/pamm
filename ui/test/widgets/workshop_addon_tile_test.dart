import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/workshop_addons.dart';
import 'package:pamm_ui/src/widgets/workshop_addons_notice.dart';

void main() {
  testWidgets('shows the addon, its pack and a link to the workshop', (
    tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: WorkshopAddonTile(
            addon: WorkshopAddonNotFound(
              name: '@cba',
              packName: 'core',
              url:
                  'https://steamcommunity.com/sharedfiles/filedetails/?id=450814997',
              steamUrl: 'steam://url/CommunityFilePage/450814997',
            ),
          ),
        ),
      ),
    );

    expect(find.text('@cba'), findsOneWidget);
    expect(find.text('Pack: core'), findsOneWidget);
    expect(find.byTooltip('Open in Steam workshop'), findsOneWidget);
  });
}
