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
      final params = await getLaunchParams(
        repotPath: widget.repotPath,
        packName: widget.packName,
      );
      if (!mounted) return;
      setState(() {
        _controller.text = params.join('\n');
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

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Enter each parameter on a new line'),
          const SizedBox(height: 16),
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

