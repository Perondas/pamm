import 'package:flutter/foundation.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../rust/api/logging.dart';

class RustLogService {
  late final Stream<String> _rustLogStream;

  final List<String> _logBuffer = [];
  String _logLevel = 'info';

  RustLogService() {
    final prefs = GetIt.instance.get<SharedPreferencesWithCache>();
    final storedLevel = prefs.getString('rust_log_level');
    if (storedLevel != null) {
      _logLevel = storedLevel;
    }

    _rustLogStream = initRustLogger(level: _logLevel).asBroadcastStream();
    if (kDebugMode) {
      _rustLogStream.listen((log) {
        print('Rust log: $log');
      });
    }

    _rustLogStream.forEach((message) {
      _logBuffer.add(message);
    });

    // Every 5 seconds cull the log buffer to the most recent 1000 entries to prevent unbounded memory growth
    Stream.periodic(const Duration(seconds: 5)).listen((_) {
      if (_logBuffer.length > 1000) {
        _logBuffer.removeRange(0, _logBuffer.length - 1000);
      }
    });
  }

  void setLogLevel(String level) {
    setRustLogLevel(level: level);
    _logLevel = level;
    _saveLogLevel(level);
  }

  void _saveLogLevel(String level) {
    final prefs = GetIt.instance.get<SharedPreferencesWithCache>();
    prefs.setString('rust_log_level', level);
  }

  List<String> get logBuffer => List.unmodifiable(_logBuffer);
  String get currentLogLevel => _logLevel;
}
