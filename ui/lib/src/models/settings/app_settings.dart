import 'package:json_annotation/json_annotation.dart';
import 'package:pamm_ui/src/models/settings/arma_settings.dart';
import 'package:pamm_ui/src/models/settings/customization_settings.dart';
import 'package:pamm_ui/src/models/settings/mm_settings.dart';

part 'app_settings.g.dart';

@JsonSerializable(explicitToJson: true)
class AppSettings {
  AppSettings({
    CustomizationSettings? customizationSettings,
    ArmaSettings? armaSettings,
    MmSettings? mmSettings,
  }) : customizationSettings = customizationSettings ?? CustomizationSettings(),
       armaSettings = armaSettings ?? ArmaSettings(),
       mmSettings = mmSettings ?? MmSettings();

  CustomizationSettings customizationSettings;
  ArmaSettings armaSettings;
  MmSettings mmSettings;

  factory AppSettings.fromJson(Map<String, dynamic> json) =>
      _$AppSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$AppSettingsToJson(this);
}
