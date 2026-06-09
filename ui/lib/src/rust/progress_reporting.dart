// Helper utilities to parse the single-StreamSink JSON events from Rust
// Do NOT modify generated files (`frb_generated.dart`). Use this helper to
// convert the raw `Stream<String>` produced by the generated bindings into
// typed events convenient for the Flutter UI.

import 'dart:async';
import 'dart:convert';

/// Base type for progress events coming from Rust.
abstract class ProgressEvent {}

/// Signifies the total amount of work (0 means unknown).
class TotalEvent extends ProgressEvent {
  final int total;
  TotalEvent(this.total);
  @override
  String toString() => 'TotalEvent(total: $total)';
}

/// Signifies incremental progress updates.
class ProgressUpdateEvent extends ProgressEvent {
  final int progress;
  ProgressUpdateEvent(this.progress);
  @override
  String toString() => 'ProgressUpdateEvent(progress: $progress)';
}

/// A human-readable message from Rust.
class MessageEvent extends ProgressEvent {
  final String message;
  MessageEvent(this.message);
  @override
  String toString() => 'MessageEvent(message: $message)';
}

/// Indicates the operation finished.
class FinishEvent extends ProgressEvent {
  FinishEvent();
  @override
  String toString() => 'FinishEvent()';
}

/// Unknown or unparseable event. Carries the raw payload for debugging.
class UnknownEvent extends ProgressEvent {
  final String raw;
  UnknownEvent(this.raw);
  @override
  String toString() => 'UnknownEvent(raw: $raw)';
}

/// Parse a single JSON-encoded event string coming from Rust into a
/// [ProgressEvent]. If parsing fails a [UnknownEvent] is returned.
ProgressEvent parseProgressEvent(String jsonStr) {
  try {
    final dynamic v = json.decode(jsonStr);
    if (v is! Map) return UnknownEvent(jsonStr);
    final Map<String, dynamic> map = Map<String, dynamic>.from(v);
    final type = map['type'];
    if (type == 'total') {
      final total = map['total'];
      if (total is int) return TotalEvent(total);
      if (total is String) return TotalEvent(int.tryParse(total) ?? 0);
      if (total is double) return TotalEvent(total.toInt());
      return UnknownEvent(jsonStr);
    }
    if (type == 'progress') {
      final progress = map['progress'];
      if (progress is int) return ProgressUpdateEvent(progress);
      if (progress is String) {
        return ProgressUpdateEvent(int.tryParse(progress) ?? 0);
      }
      if (progress is double) return ProgressUpdateEvent(progress.toInt());
      return UnknownEvent(jsonStr);
    }
    if (type == 'message') {
      final message = map['message'];
      if (message is String) return MessageEvent(message);
      return UnknownEvent(jsonStr);
    }
    if (type == 'finish') {
      return FinishEvent();
    }
    return UnknownEvent(jsonStr);
  } catch (e) {
    return UnknownEvent(jsonStr);
  }
}

/// Convert the raw `Stream<String>` (as provided by the generated FRB bindings)
/// into a `Stream<ProgressEvent>` with parsed, typed events. Invalid or
/// unparseable messages are emitted as `UnknownEvent` so the UI can log or
/// ignore them.
Stream<ProgressEvent> progressEventStreamFromRaw(Stream<String> raw) {
  return raw.map(parseProgressEvent);
}

/// Convenience: create a broadcast controller to be passed to Rust as the
/// single `StreamSink<String>` and to be listened to as parsed events.
///
/// Usage:
/// ```dart
/// final controller = createProgressController();
/// // Pass controller.sink to the generated Rust binding, e.g.:
/// // await rustApi.createDartProgressReporter(controller.sink);
/// final parsed = progressEventStreamFromRaw(controller.stream);
/// parsed.listen((evt) { ... });
/// ```
StreamController<String> createProgressController({bool broadcast = false}) {
  return broadcast
      ? StreamController<String>.broadcast()
      : StreamController<String>();
}
