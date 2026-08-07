import 'package:json_annotation/json_annotation.dart';
import 'package:pamm_ui/src/models/settings/arma_settings.dart';
import 'package:pamm_ui/src/models/settings/customization_settings.dart';

part 'app_settings.g.dart';

@JsonSerializable(explicitToJson: true)
class AppSettings {
  AppSettings({
    CustomizationSettings? customizationSettings,
    ArmaSettings? armaSettings,
  }) : customizationSettings = customizationSettings ?? CustomizationSettings(),
       armaSettings = armaSettings ?? ArmaSettings();

  CustomizationSettings customizationSettings;
  ArmaSettings armaSettings;

  factory AppSettings.fromJson(Map<String, dynamic> json) =>
      _$AppSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$AppSettingsToJson(this);
}
