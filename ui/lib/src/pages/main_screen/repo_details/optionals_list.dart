import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/optionals/load_optionals.dart';
import 'package:pamm_ui/src/rust/api/commands/optionals/save_optionals.dart';

class OptionalsList extends StatefulWidget {
  const OptionalsList(this.repotPath, this.packName, {super.key});

  final String repotPath;
  final String packName;

  @override
  State<OptionalsList> createState() => _OptionalsListState();
}

class _OptionalsListState extends State<OptionalsList> {
  bool loading = true;
  List<OptionalAddon> optionals = [];

  @override
  void initState() {
    super.initState();
    _loadOptionals();
  }

  Future<void> _loadOptionals() async {
    setState(() => loading = true);
    try {
      final optionals = await loadOptionals(
        repotPath: widget.repotPath,
        packName: widget.packName,
      );
      optionals.sort((a, b) => a.name.compareTo(b.name),);
      if (!mounted) return;
      setState(() {
        this.optionals = optionals;
        loading = false;
      });
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to load optionals: $e')));
    }
  }

  Future<void> _saveOptionals() async {
    try {
      await saveOptionals(
        repoPath: widget.repotPath,
        packName: widget.packName,
        optionals: optionals,
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to save optionals: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          Text("Optionals", style: Theme.of(context).textTheme.headlineSmall),
          if (loading) CircularProgressIndicator(),
          for (int i = 0; i < optionals.length; i++)
            SwitchListTile(
              title: Text(optionals[i].name),
              value: optionals[i].enabled,
              onChanged: (value) {
                setState(() {
                  optionals.setRange(i, i + 1, [
                    OptionalAddon(name: optionals[i].name, enabled: value),
                  ]);
                });
                _saveOptionals();
              },
            ),
        ],
      ),
    );
  }
}
