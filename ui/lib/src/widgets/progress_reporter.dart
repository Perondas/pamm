import 'dart:async';

import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
import 'package:pamm_ui/src/services/progress_reporter_service.dart';

class ProgressReporter extends StatefulWidget {
  const ProgressReporter(this.service, {super.key});

  final ProgressReporterService service;

  @override
  State<ProgressReporter> createState() => _ProgressReporterState();
}

class _Sample {
  _Sample(this.time, this.bytes);

  final DateTime time;
  final BigInt bytes;
}

class _ProgressReporterState extends State<ProgressReporter> {
  bool isFinished = false;
  BigInt? total;
  BigInt progress = BigInt.zero;
  final List<String> messages = [];
  final ScrollController _scrollController = ScrollController();

  // Sliding window of recent progress samples used to estimate the current
  // download speed (and from it, the time remaining).
  static const Duration _speedWindow = Duration(seconds: 5);
  final List<_Sample> _samples = [];
  double? _bytesPerSecond;

  // The Rust side reports progress every 1 KiB, which for a real download is a
  // flood of events. Rebuilding on each one saturates the UI isolate and makes
  // the bar stutter, so we accumulate progress silently and repaint on a timer.
  static const Duration _refreshInterval = Duration(seconds: 1);
  Timer? _refreshTimer;

  @override
  void initState() {
    super.initState();
    widget.service.isFinished.listen(
      onChange: (value) {
        if (!mounted) return;
        setState(() {
          isFinished = value;
        });
      },
    );
    widget.service.total.listen(
      onChange: (value) {
        if (!mounted) return;
        setState(() {
          total = value;
        });
      },
    );
    // Accumulate incoming progress without rebuilding — the periodic timer
    // below is what actually repaints, at most once per second.
    widget.service.progressStream.listen((progressValue) {
      if (!mounted) return;
      if (isFinished) {
        isFinished = false;
        progress = BigInt.zero;
        _samples.clear();
        _bytesPerSecond = null;
      }
      progress = progress + progressValue;
    });

    // Recompute the speed/ETA estimate and repaint once per second.
    _refreshTimer = Timer.periodic(_refreshInterval, (_) {
      if (!mounted) return;
      setState(_updateSpeed);
    });
    widget.service.messageStream.listen((message) {
      if (!mounted) return;
      if (message.isNotEmpty) {
        setState(() {
          messages.add(message);
        });
        // Auto-scroll to bottom
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (_scrollController.hasClients) {
            _scrollController.animateTo(
              _scrollController.position.maxScrollExtent,
              duration: const Duration(milliseconds: 200),
              curve: Curves.easeOut,
            );
          }
        });
      }
    });
  }

  @override
  void dispose() {
    _refreshTimer?.cancel();
    _scrollController.dispose();
    super.dispose();
  }

  /// Records the latest progress sample and recomputes the download speed over
  /// the trailing [_speedWindow].
  void _updateSpeed() {
    final now = DateTime.now();
    _samples.add(_Sample(now, progress));
    // Keep at least two samples so a speed can be computed, while dropping
    // anything older than the window.
    while (_samples.length > 2 &&
        now.difference(_samples.first.time) > _speedWindow) {
      _samples.removeAt(0);
    }

    final first = _samples.first;
    final elapsedMs = now.difference(first.time).inMilliseconds;
    if (elapsedMs > 0) {
      final deltaBytes = (progress - first.bytes).toDouble();
      _bytesPerSecond = deltaBytes * 1000 / elapsedMs;
    }
  }

  /// Builds the `<speed>/s • <time> left` label shown next to the progress
  /// text, or null when no meaningful speed is available yet.
  String? _speedAndEtaLabel() {
    final speed = _bytesPerSecond;
    if (speed == null || speed <= 0) return null;

    final label = StringBuffer('${format(speed.round())}/s');
    final t = total;
    if (t != null && t > progress) {
      final remaining = (t - progress).toDouble();
      label.write(' • ${_formatDuration(remaining / speed)} left');
    }
    return label.toString();
  }

  /// Formats a number of seconds into a short human-readable duration.
  String _formatDuration(double seconds) {
    final d = Duration(seconds: seconds.round());
    final h = d.inHours;
    final m = d.inMinutes.remainder(60);
    final s = d.inSeconds.remainder(60);
    if (h > 0) return '${h}h ${m}m';
    if (m > 0) return '${m}m ${s}s';
    return '${s}s';
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colors = theme.colorScheme;
    final t = total;
    final indeterminate = t == BigInt.zero;

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        if (!isFinished && t != null) ...[
          if (indeterminate)
            Text('Progress: $progress', style: theme.textTheme.titleMedium)
          else
            Row(
              crossAxisAlignment: CrossAxisAlignment.baseline,
              textBaseline: TextBaseline.alphabetic,
              children: [
                Text(
                  '${(progress.toDouble() / t.toDouble() * 100).clamp(0, 100).toStringAsFixed(0)}%',
                  style: theme.textTheme.headlineMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(width: 16),
                Text(
                  '${format(progress.toInt())} / ${format(t.toInt())}',
                  style: theme.textTheme.bodyLarge,
                ),
                const Spacer(),
                if (_speedAndEtaLabel() case final label?)
                  Text(
                    label,
                    style: theme.textTheme.bodyMedium?.copyWith(
                      color: colors.onSurfaceVariant,
                    ),
                  ),
              ],
            ),
          const SizedBox(height: 12),
          if (indeterminate)
            const LinearProgressIndicator(
              value: null,
              minHeight: 12,
              borderRadius: BorderRadius.all(Radius.circular(6)),
            )
          else
            // Animate between the once-per-second value updates so the bar
            // glides smoothly instead of stepping.
            TweenAnimationBuilder<double>(
              tween: Tween<double>(end: progress.toDouble() / t.toDouble()),
              duration: _refreshInterval,
              curve: Curves.linear,
              builder: (context, value, _) => LinearProgressIndicator(
                value: value,
                minHeight: 12,
                borderRadius: const BorderRadius.all(Radius.circular(6)),
              ),
            ),
          const SizedBox(height: 24),
        ],

        // Scrolling messages view
        if (messages.isNotEmpty) ...[
          Text("Logs", style: theme.textTheme.titleMedium),
          const SizedBox(height: 8),
          Expanded(
            child: Container(
              decoration: BoxDecoration(
                color: colors.surfaceContainerLowest,
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: colors.outlineVariant),
              ),
              child: ListView.builder(
                controller: _scrollController,
                padding: const EdgeInsets.all(12),
                itemCount: messages.length,
                itemBuilder: (context, index) {
                  return SelectableText(
                    messages[index],
                    style: theme.textTheme.bodySmall?.copyWith(
                      fontFamily: 'monospace',
                      color: colors.onSurfaceVariant,
                    ),
                  );
                },
              ),
            ),
          ),
        ],
      ],
    );
  }
}
