import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/params.dart';

class ParamsForm extends StatefulWidget {
  const ParamsForm(this.repotPath, this.packName, {super.key});

  final String repotPath;
  final String packName;

  @override
  State<ParamsForm> createState() => _ParamsFormState();
}

class _ParamsFormState extends State<ParamsForm> {
  bool loading = true;
  List<String> _serverParams = [];
  final TextEditingController _controller = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadParams();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _loadParams() async {
    setState(() => loading = true);
    try {
      final results = await Future.wait([
        getServerLaunchParams(
          repotPath: widget.repotPath,
          packName: widget.packName,
        ),
        getLaunchParams(
          repotPath: widget.repotPath,
          packName: widget.packName,
        ),
      ]);
      if (!mounted) return;
      setState(() {
        _serverParams = results[0];
        _controller.text = results[1].join('\n');
        loading = false;
      });
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to load params: $e')));
    }
  }

  Future<void> _saveParams() async {
    try {
      // Split by newline and trim, ignoring empty lines
      final params = _controller.text
          .split('\n')
          .map((e) => e.trim())
          .where((e) => e.isNotEmpty)
          .toList();

      await setLaunchParams(
        repotPath: widget.repotPath,
        packName: widget.packName,
        launchParams: params,
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to save params: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return const Center(child: CircularProgressIndicator());
    }

    final theme = Theme.of(context);

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Pack parameters', style: theme.textTheme.titleSmall),
          const SizedBox(height: 4),
          Text(
            'Defined by the pack and always applied at launch.',
            style: theme.textTheme.bodySmall,
          ),
          const SizedBox(height: 8),
          Container(
            width: double.infinity,
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: theme.colorScheme.surfaceContainerHighest,
              border: Border.all(color: theme.colorScheme.outlineVariant),
              borderRadius: BorderRadius.circular(4),
            ),
            child: SelectableText(
              _serverParams.isEmpty ? '(none)' : _serverParams.join('\n'),
              style: theme.textTheme.bodyMedium?.copyWith(
                fontFamily: 'monospace',
                color: _serverParams.isEmpty
                    ? theme.colorScheme.onSurfaceVariant
                    : null,
              ),
            ),
          ),
          const SizedBox(height: 16),
          Text('Your parameters', style: theme.textTheme.titleSmall),
          const SizedBox(height: 4),
          Text(
            'Enter each parameter on a new line.',
            style: theme.textTheme.bodySmall,
          ),
          const SizedBox(height: 8),
          Expanded(
            child: TextField(
              controller: _controller,
              maxLines: null,
              expands: true,
              textAlignVertical: TextAlignVertical.top,
              decoration: const InputDecoration(
                border: OutlineInputBorder(),
              ),
              onChanged: (_) => _saveParams(),
            ),
          ),
        ],
      ),
    );
  }
}

