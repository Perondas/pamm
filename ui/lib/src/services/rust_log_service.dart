import 'package:flutter/foundation.dart';

import '../rust/api/logging.dart';

class RustLogService {
  late final Stream<String> _rustLogStream;

  final List<String> _logBuffer = [];
  String _logLevel = 'debug';

  RustLogService() {
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
  }

  List<String> get logBuffer => List.unmodifiable(_logBuffer);
  String get currentLogLevel => _logLevel;
}
