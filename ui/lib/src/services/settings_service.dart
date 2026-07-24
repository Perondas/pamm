import 'package:get_it/get_it.dart';
import 'package:pamm_ui/src/models/settings/app_settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

class SettingService {
  static const _key = 'user_settings_v1';

  // Load all stored repos
  static Future<AppSettings> getSettings() async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    final raw = prefs.getString(_key);
    final settings = AppSettings.fromJson(
      raw != null ? Map<String, dynamic>.from(Uri.splitQueryString(raw)) : {},
    );
    return settings;
  }

  static Future<void> saveSettigs(AppSettings settings) async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    await prefs.setString(
      _key,
      Uri.encodeQueryComponent(settings.toJson().toString()),
    );
  }
}
