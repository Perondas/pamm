import 'package:flutter/material.dart';
import 'package:pamm_ui/src/models/settings/customization_settings.dart';
import 'package:pamm_ui/src/rust/api/commands/init_from_remote.dart';
import 'package:pamm_ui/src/services/settings_service.dart';

const defaultSeedColor = Color.fromARGB(255, 236, 214, 153);

class ThemeService extends ChangeNotifier {
  static final ThemeService _instance = ThemeService._internal();

  factory ThemeService() {
    return _instance;
  }

  ThemeService._internal();

  Color? _repoColor;

  /// Adopt the custom color of the selected repo, if it defines one.
  /// Not persisted; cleared when no repo is selected.
  void applyRepo(RepoConfig? repo) {
    final color = _colorFromConfig(repo?.customization?.color);
    if (_repoColor == color) return;
    _repoColor = color;
    notifyListeners();
  }

  /// The seed color the app theme should currently be generated from.
  /// The selected repo's color wins unless the user fixed the seed color
  /// in the settings.
  Color get seedColor {
    final customization = settingsService.settings.customizationSettings;
    if (!customization.fixedSeedColor && _repoColor != null) {
      return _repoColor!;
    }
    return customization.seedColor ?? defaultSeedColor;
  }

  ThemeMode get themeMode =>
      switch (settingsService.settings.customizationSettings.theme) {
        ThemePreference.system => ThemeMode.system,
        ThemePreference.light => ThemeMode.light,
        ThemePreference.dark => ThemeMode.dark,
      };

  /// Repo configs store colors as an (a, r, g, b) tuple.
  static Color? _colorFromConfig((int, int, int, int)? color) {
    if (color == null) return null;
    final (a, r, g, b) = color;
    return Color.fromARGB(a, r, g, b);
  }
}

final themeService = ThemeService();
