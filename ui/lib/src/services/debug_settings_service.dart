class DebugSettingsService {
  bool alwaysForceRefresh = false;

  static final DebugSettingsService _instance = DebugSettingsService._internal();

  factory DebugSettingsService() {
    return _instance;
  }

  DebugSettingsService._internal();
}

final debugSettingsService = DebugSettingsService();

