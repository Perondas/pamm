import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:get_it/get_it.dart';
import 'package:pamm_ui/src/models/settings/app_settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

class SettingsService extends ChangeNotifier {
  static const _key = 'user_settings_v1';

  static final SettingsService _instance = SettingsService._internal();

  factory SettingsService() {
    return _instance;
  }

  SettingsService._internal();

  AppSettings _settings = AppSettings();

  AppSettings get settings => _settings;

  Future<void> load() async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    final raw = prefs.getString(_key);
    if (raw == null) return;
    try {
      _settings = AppSettings.fromJson(jsonDecode(raw) as Map<String, dynamic>);
    } catch (_) {
      // Corrupt or outdated payload; fall back to defaults.
      _settings = AppSettings();
    }
    notifyListeners();
  }

  Future<void> update(void Function(AppSettings settings) change) async {
    change(_settings);
    notifyListeners();
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    await prefs.setString(_key, jsonEncode(_settings.toJson()));
  }
}

final settingsService = SettingsService();
