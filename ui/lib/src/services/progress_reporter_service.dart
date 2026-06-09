import 'package:dart_observable/dart_observable.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:pamm_ui/src/rust/api/progress_reporting.dart';
import 'package:pamm_ui/src/rust/progress_reporting.dart' as pr;

class ProgressReporterService {
  late DartProgressReporter _underlyingReporter;
  late RustStreamSink<String> _sink;

  late Observable<BigInt?> _total;
  late Stream<BigInt> _progressStream;
  late Stream<String> _messageStream;
  late Observable<bool> _finished;

  ProgressReporterService() {
    _sink = RustStreamSink<String>();

    _underlyingReporter = createDartProgressReporter(
      sink: _sink,
      dummy: RustStreamSink(),
    );

    // Convert the raw JSON string stream into typed events and then expose
    // filtered streams for UI consumers.
    final parsed = pr.progressEventStreamFromRaw(_sink.stream.asBroadcastStream());

    _total = Observable<BigInt?>.fromStream(
      initial: null,
      stream: parsed
          .where((e) => e is pr.TotalEvent)
          .map((e) => BigInt.from((e as pr.TotalEvent).total)),
    );

    _progressStream = parsed
        .where((e) => e is pr.ProgressUpdateEvent)
        .map((e) => BigInt.from((e as pr.ProgressUpdateEvent).progress))
        .asBroadcastStream();

    _messageStream = parsed
        .where((e) => e is pr.MessageEvent)
        .map((e) => (e as pr.MessageEvent).message)
        .asBroadcastStream();

    _finished = Observable<bool>.fromStream(
      initial: false,
      stream: parsed.where((e) => e is pr.FinishEvent).map((_) => true),
    );
  }

  DartProgressReporter get underlyingReporter => _underlyingReporter;

  Stream<BigInt> get progressStream => _progressStream;

  Observable<BigInt?> get total => _total;

  Stream<String> get messageStream => _messageStream;

  Observable<bool> get isFinished => _finished;
}
