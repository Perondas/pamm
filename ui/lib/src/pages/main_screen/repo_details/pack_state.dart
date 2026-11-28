import 'package:flutter/foundation.dart';
import 'package:pamm_ui/src/rust/api/commands/load_pack_display.dart';
import 'package:pamm_ui/src/rust/api/commands/load_pack_info.dart';
import 'package:pamm_ui/src/rust/api/commands/pack_sync/quick_check.dart';

/// Holds the async-loaded state of one pack (sync status, display info and
/// config), shared by the pack sidebar and the pack detail pane.
class PackStateManager with ChangeNotifier {
  PackStateManager({
    required this.packName,
    required this.repoPath,
    required this.isLocal,
    required this.onError,
  }) {
    checkStatus();
    _loadDisplay();
    _loadConfig();
  }

  final String packName;
  final String repoPath;
  final bool isLocal;

  /// Reports errors that the user should see (e.g. as a snackbar).
  final ValueChanged<String> onError;

  bool? upToDate;
  PackDisplayInfo? display;
  PackInfo? info;

  bool _disposed = false;

  Future<void> _loadConfig() async {
    try {
      final info = await loadPackInfo(repoPath: repoPath, packName: packName);
      if (_disposed) return;
      this.info = info;
      notifyListeners();
    } catch (e) {
      if (_disposed) return;
      onError('Failed to load pack info: $e');
    }
  }

  Future<void> _loadDisplay() async {
    try {
      final display = await loadPackDisplay(
        repoPath: repoPath,
        packName: packName,
      );
      if (_disposed) return;
      this.display = display;
      notifyListeners();
    } catch (e) {
      // Media is cosmetic; fall back to the letter avatar.
    }
  }

  Future<void> checkStatus() async {
    try {
      final upToDate = await quickCheck(packName: packName, repoPath: repoPath);
      if (_disposed) return;
      this.upToDate = upToDate;
      notifyListeners();
    } catch (e) {
      // Ignore errors for quick check
    }
  }

  @override
  void dispose() {
    _disposed = true;
    super.dispose();
  }
}
