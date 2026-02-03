import 'package:flutter/material.dart';
import 'package:ui/src/services/progress_reporter_service.dart';

class ProgressReporter extends StatefulWidget {
  const ProgressReporter(this.service, {super.key});

  final ProgressReporterService service;

  @override
  State<ProgressReporter> createState() => _ProgressReporterState();
}

class _ProgressReporterState extends State<ProgressReporter> {
  bool isFinished = false;
  BigInt? total;
  final List<String> messages = [];
  final ScrollController _scrollController = ScrollController();

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
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (isFinished) {
      return const Text('Completed');
    }

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Scrolling messages view
        if (messages.isNotEmpty)
          Container(
            height: 120,
            width: double.infinity,
            decoration: BoxDecoration(
              border: Border.all(color: Colors.grey.shade300),
              borderRadius: BorderRadius.circular(4),
            ),
            child: ListView.builder(
              controller: _scrollController,
              padding: const EdgeInsets.all(8),
              itemCount: messages.length,
              itemBuilder: (context, index) {
                return Text(
                  messages[index],
                  style: Theme.of(context).textTheme.bodySmall,
                );
              },
            ),
          ),
        const SizedBox(height: 16),
        // Progress section
        if (total != null)
          StreamBuilder<BigInt>(
            stream: widget.service.progressStream,
            builder: (context, progressSnapshot) {
              final progress = progressSnapshot.data ?? BigInt.zero;

              if (total == BigInt.zero) {
                // Indeterminate progress with actual number display
                return Column(
                  children: [
                    Text('Progress: ${progress.toString()}'),
                    const SizedBox(height: 8),
                    const LinearProgressIndicator(), // Indeterminate
                  ],
                );
              } else {
                // Determinate progress
                final progressValue = progress.toDouble() / total!.toDouble();
                return Column(
                  children: [
                    Text('${progress.toString()} / ${total.toString()}'),
                    const SizedBox(height: 8),
                    LinearProgressIndicator(value: progressValue.clamp(0.0, 1.0)),
                  ],
                );
              }
            },
          ),
      ],
    );
  }
}
