import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:ui/src/rust/api/progress_reporting.dart';

class ProgressReporterService {
  late DartProgressReporter _underlyingReporter;
  late RustStreamSink<String> totalSink;
  late RustStreamSink<String> progressSink;
  late RustStreamSink<String> messageSink;
  late RustStreamSink<bool> finishSink;

  late Stream<BigInt> _totalStream;
  late Stream<BigInt> _progressStream;
  late Stream<String> _messageStream;
  late Stream<bool> _finishStream;

  ProgressReporterService() {
    totalSink = RustStreamSink<String>();
    progressSink = RustStreamSink<String>();
    messageSink = RustStreamSink<String>();
    finishSink = RustStreamSink<bool>();

    _underlyingReporter = createDartProgressReporter(
      reportTotalSink: totalSink,
      reportProgressSink: progressSink,
      messageSink: messageSink,
      finishSink: finishSink,
    );

    _totalStream = totalSink.stream
        .map((value) => BigInt.parse(value))
        .asBroadcastStream();
    _progressStream = progressSink.stream
        .map((value) => BigInt.parse(value))
        .asBroadcastStream();
    _messageStream = messageSink.stream.map((s) {
      print(s);
      return s;
    }).asBroadcastStream();
    _finishStream = finishSink.stream.asBroadcastStream();
  }

  DartProgressReporter get underlyingReporter => _underlyingReporter;

  Stream<BigInt> get progressStream => _progressStream;

  Stream<BigInt> get totalStream => _totalStream;

  Stream<String> get messageStream => _messageStream;

  Stream<bool> get finishStream => _finishStream;
}
