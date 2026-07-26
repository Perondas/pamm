import 'package:json_annotation/json_annotation.dart';
import 'package:pamm_ui/src/models/settings/customization_settings.dart';

part 'app_settings.g.dart';

@JsonSerializable(explicitToJson: true)
class AppSettings {
  AppSettings({CustomizationSettings? customizationSettings})
    : customizationSettings = customizationSettings ?? CustomizationSettings();

  CustomizationSettings customizationSettings;

  factory AppSettings.fromJson(Map<String, dynamic> json) =>
      _$AppSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$AppSettingsToJson(this);
}
