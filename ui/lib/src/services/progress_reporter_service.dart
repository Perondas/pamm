import 'package:dart_observable/dart_observable.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:ui/src/rust/api/progress_reporting.dart';

class ProgressReporterService {
  late DartProgressReporter _underlyingReporter;
  late RustStreamSink<String> _totalSink;
  late RustStreamSink<String> _progressSink;
  late RustStreamSink<String> _messageSink;
  late RustStreamSink<bool> _finishSink;

  late Stream<BigInt> _totalStream;
  late Stream<BigInt> _progressStream;
  late Stream<String> _messageStream;
  late Observable<bool> _finished;

  ProgressReporterService() {
    _totalSink = RustStreamSink<String>();
    _progressSink = RustStreamSink<String>();
    _messageSink = RustStreamSink<String>();
    _finishSink = RustStreamSink<bool>();

    _underlyingReporter = createDartProgressReporter(
      reportTotalSink: _totalSink,
      reportProgressSink: _progressSink,
      messageSink: _messageSink,
      finishSink: _finishSink,
    );

    _totalStream = _totalSink.stream
        .map((value) => BigInt.parse(value))
        .asBroadcastStream();
    _progressStream = _progressSink.stream
        .map((value) => BigInt.parse(value))
        .asBroadcastStream();
    _messageStream = _messageSink.stream.map((s) {
      return s;
    }).asBroadcastStream();
    _finished = Observable<bool>.fromStream(
      initial: false,
      stream: _finishSink.stream,
    );
  }

  DartProgressReporter get underlyingReporter => _underlyingReporter;

  Stream<BigInt> get progressStream => _progressStream;

  Stream<BigInt> get totalStream => _totalStream;

  Stream<String> get messageStream => _messageStream;

  Observable<bool> get isFinished => _finished;
}
