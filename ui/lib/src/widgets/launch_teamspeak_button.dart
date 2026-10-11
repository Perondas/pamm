import 'package:flutter/material.dart';
import 'package:pamm_ui/src/rust/api/commands/proton.dart';

/// Launches TeamSpeak in Arma's Proton prefix on Linux. Renders nothing when
/// TeamSpeak is not installed, or there is no prefix yet.
class LaunchTeamspeakButton extends StatefulWidget {
  const LaunchTeamspeakButton({super.key});

  @override
  State<LaunchTeamspeakButton> createState() => _LaunchTeamspeakButtonState();
}

class _LaunchTeamspeakButtonState extends State<LaunchTeamspeakButton> {
  bool _installed = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final info = await protonInfo();
      if (!mounted) return;
      setState(() => _installed = info.teamspeakExe != null);
    } catch (_) {
      // No prefix yet; the Proton dialog explains why.
    }
  }

  Future<void> _launch() async {
    try {
      await protonLaunchTeamspeak();
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text("$e")));
    }
  }

  @override
  Widget build(BuildContext context) {
    if (!_installed) return const SizedBox.shrink();

    return OutlinedButton.icon(
      onPressed: _launch,
      icon: const Icon(Icons.headset_mic),
      label: const Text("Launch TeamSpeak"),
    );
  }
}
