import 'package:flutter/material.dart';
import 'package:ui/src/services/progress_reporter_service.dart';

class ProgressReporter extends StatefulWidget {
  const ProgressReporter(this.service, {super.key});

  final ProgressReporterService service;

  @override
  State<ProgressReporter> createState() => _ProgressReporterState();
}

class _ProgressReporterState extends State<ProgressReporter> {
  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: widget.service.messageStream,
      builder: (context, snapshot) {
        final message = snapshot.data ?? 'Starting...';
        return Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(message.toString()),
            const SizedBox(height: 8),
            StreamBuilder<BigInt>(
              stream: widget.service.progressStream,
              builder: (context, progressSnapshot) {
                final progress = progressSnapshot.data ?? BigInt.zero;
                return StreamBuilder<BigInt>(
                  stream: widget.service.totalStream,
                  builder: (context, totalSnapshot) {
                    final total = totalSnapshot.data ?? BigInt.one;
                    double progressValue = 0.0;
                    if (total > BigInt.zero) {
                      progressValue = progress.toDouble() / total.toDouble();
                    }
                    return LinearProgressIndicator(value: progressValue);
                  },
                );
              },
            ),
          ],
        );
      },
    );
  }
}
