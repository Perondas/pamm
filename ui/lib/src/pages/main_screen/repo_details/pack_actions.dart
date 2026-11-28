import 'package:flutter/material.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/edit_pack_dialog.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_details/pack_state.dart';
import 'package:pamm_ui/src/pages/sync_screen/main.dart';
import 'package:pamm_ui/src/pages/sync_single_pack_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/delete_pack_content.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';
import 'package:pamm_ui/src/rust/api/commands/local_pack/remove_local_pack.dart';
import 'package:pamm_ui/src/rust/api/commands/local_pack/sync_local_pack.dart';
import 'package:pamm_ui/src/services/debug_settings_service.dart';
import 'package:pamm_ui/src/services/settings_service.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';
import 'package:pamm_ui/src/widgets/workshop_addons_notice.dart';

void _showError(BuildContext context, String message) {
  ScaffoldMessenger.of(
    context,
  ).showSnackBar(SnackBar(content: Text(message), backgroundColor: Colors.red));
}

Future<void> launchPack(BuildContext context, PackStateManager pack) async {
  try {
    await launch(
      repoDir: pack.repoPath,
      packName: pack.packName,
      launchType: settingsService.settings.armaSettings.launchType,
      disableOptionals:
          settingsService.settings.mmSettings.mmModeEnabled &&
          settingsService.settings.mmSettings.launchWithoutOptionals,
    );
  } catch (e) {
    if (!context.mounted) return;
    _showError(context, "Error launching pack: $e");
  }
}

Future<void> syncPack(BuildContext context, PackStateManager pack) async {
  if (pack.isLocal) {
    try {
      await syncLocalPack(packName: pack.packName, repoPath: pack.repoPath);
      if (!context.mounted) return;
      await showWorkshopAddonsDialogIfNeeded(
        context,
        repoPath: pack.repoPath,
        packName: pack.packName,
      );
    } catch (e) {
      if (!context.mounted) return;
      _showError(context, "Error syncing pack: $e");
    }
  } else {
    await Navigator.of(context).push(
      MaterialPageRoute(
        builder: (context) => debugSettingsService.useLegacySinglePackSync
            ? SyncSinglePackScreen(pack.packName, pack.repoPath)
            : SyncScreen(pack.packName, pack.repoPath),
      ),
    );
    // Re-check status after returning from sync
    pack.checkStatus();
  }
}

Future<void> editPack(BuildContext context, PackStateManager pack) async {
  await showDialog(
    context: context,
    builder: (_) => EditPackDialog(pack.repoPath, pack.packName),
  );
}

Future<void> clearPackContent(
  BuildContext context,
  PackStateManager pack,
) async {
  final answer = await showDialog<bool?>(
    context: context,
    builder: (_) => ConfirmDialog(
      content:
          'Are you sure you want to clear the content of ${pack.packName}? This will delete all addons on disk!',
      title: "Clear ${pack.packName}",
    ),
  );

  if (!context.mounted || answer != true) return;

  await deletePackContent(repoPath: pack.repoPath, packName: pack.packName);

  // Show a snackbar on success
  if (!context.mounted) return;
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(content: Text("Content of ${pack.packName} cleared from disk")),
  );
}

/// Deletes a local pack (after confirmation). [onDone] runs afterwards
/// whether or not the user confirmed, so the caller can reload.
Future<void> deleteLocalPack(
  BuildContext context,
  PackStateManager pack,
  VoidCallback onDone,
) async {
  try {
    final confirm =
        await showDialog<bool?>(
          context: context,
          builder: (_) => ConfirmDialog(
            content:
                'Are you sure you want to delete the pack "${pack.packName}"? This WILL delete files on disk.',
          ),
        ) ??
        false;

    if (confirm) {
      await removeLocalPack(repoPath: pack.repoPath, packName: pack.packName);
    }
    onDone();
  } catch (e) {
    if (!context.mounted) return;
    _showError(context, "Error deleting pack: $e");
  }
}
