import 'package:flutter/material.dart';
import 'package:format_bytes/format_bytes.dart';
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
  BigInt progress = BigInt.zero;
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
    widget.service.progressStream.listen((progressValue) {
      if (!mounted) return;
      setState(() {
        progress = progress + progressValue;
      });
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
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Progress section
        if (!isFinished && total != null) ...[
          if (total == BigInt.zero) ...[
            Column(
              children: [
                Text('Progress: ${progress.toString()}'),
                const SizedBox(height: 8),
                const LinearProgressIndicator(value: null, minHeight: 10),
              ],
            ),
          ] else ...[
            Column(
              children: [
                Text('${format(progress.toInt())} / ${format(total!.toInt())}'),
                const SizedBox(height: 8),
                LinearProgressIndicator(
                  value: progress.toDouble() / total!.toDouble(),
                  minHeight: 10,
                ),
              ],
            ),
          ],
        ],
        const SizedBox(height: 16),

        // Scrolling messages view
        if (messages.isNotEmpty)
          Container(
            height: 200,
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
      ],
    );
  }
}
