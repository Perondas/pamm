import 'package:flutter/material.dart';
import 'package:pamm_ui/main.dart';
import 'package:pamm_ui/src/pages/settings_screen/settings_group.dart';
import 'package:pamm_ui/src/services/settings_service.dart';

class CustomizationGroup extends StatefulWidget {
  const CustomizationGroup({super.key});

  @override
  State<CustomizationGroup> createState() => _CustomizationGroupState();
}

class _CustomizationGroupState extends State<CustomizationGroup> {
  @override
  Widget build(BuildContext context) {
    final customization = settingsService.settings.customizationSettings;
    return SettingsGroup(
      title: "Customization",
      children: [
        ListTile(
          leading: Icon(Icons.palette),
          title: Text("Custom seed color"),
          subtitle: Text("Overrides the default theme seed color"),
          trailing: customization.seedColor != null
              ? CircleAvatar(radius: 14, backgroundColor: customization.seedColor)
              : Text("Default"),
          onTap: () async {
            final picked = await showDialog<Color>(
              context: context,
              builder: (context) => _ColorPickerDialog(
                initial: customization.seedColor ?? defaultSeedColor,
              ),
            );
            if (picked == null) return;
            await settingsService.update(
              (settings) => settings.customizationSettings.seedColor = picked,
            );
            setState(() {});
          },
        ),
        if (customization.seedColor != null)
          ListTile(
            leading: Icon(Icons.restart_alt),
            title: Text("Remove custom seed color"),
            onTap: () async {
              await settingsService.update(
                (settings) => settings.customizationSettings.seedColor = null,
              );
              setState(() {});
            },
          ),
        SwitchListTile(
          secondary: Icon(Icons.push_pin_outlined),
          title: Text("Fix seed color"),
          value: customization.fixedSeedColor,
          onChanged: (val) async {
            await settingsService.update(
              (settings) => settings.customizationSettings.fixedSeedColor = val,
            );
            setState(() {});
          },
        ),
      ],
    );
  }
}

class _ColorPickerDialog extends StatefulWidget {
  const _ColorPickerDialog({required this.initial});

  final Color initial;

  @override
  State<_ColorPickerDialog> createState() => _ColorPickerDialogState();
}

class _ColorPickerDialogState extends State<_ColorPickerDialog> {
  static final _presets = <Color>[defaultSeedColor, ...Colors.primaries];

  late int _red;
  late int _green;
  late int _blue;

  Color get _color => Color.fromARGB(255, _red, _green, _blue);

  @override
  void initState() {
    super.initState();
    _setColor(widget.initial);
  }

  void _setColor(Color color) {
    _red = (color.r * 255).round();
    _green = (color.g * 255).round();
    _blue = (color.b * 255).round();
  }

  Widget _channelSlider(String label, int value, void Function(int) onChanged) {
    return Row(
      children: [
        SizedBox(width: 16, child: Text(label)),
        Expanded(
          child: Slider(
            value: value.toDouble(),
            max: 255,
            onChanged: (val) => setState(() => onChanged(val.round())),
          ),
        ),
        SizedBox(width: 32, child: Text('$value', textAlign: TextAlign.right)),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text("Pick a seed color"),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: _presets.map((preset) {
                final selected = preset.toARGB32() == _color.toARGB32();
                return InkWell(
                  customBorder: CircleBorder(),
                  onTap: () => setState(() => _setColor(preset)),
                  child: CircleAvatar(
                    radius: 16,
                    backgroundColor: preset,
                    child: selected ? Icon(Icons.check, size: 18) : null,
                  ),
                );
              }).toList(),
            ),
            SizedBox(height: 16),
            Row(
              children: [
                CircleAvatar(radius: 16, backgroundColor: _color),
                SizedBox(width: 12),
                Text(
                  '#${_color.toARGB32().toRadixString(16).padLeft(8, '0').substring(2).toUpperCase()}',
                  style: TextStyle(fontFamily: 'monospace'),
                ),
              ],
            ),
            _channelSlider('R', _red, (val) => _red = val),
            _channelSlider('G', _green, (val) => _green = val),
            _channelSlider('B', _blue, (val) => _blue = val),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: Text("Cancel"),
        ),
        FilledButton(
          onPressed: () => Navigator.of(context).pop(_color),
          child: Text("Apply"),
        ),
      ],
    );
  }
}
