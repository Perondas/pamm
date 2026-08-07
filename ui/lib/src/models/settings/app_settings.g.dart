// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'app_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AppSettings _$AppSettingsFromJson(Map<String, dynamic> json) => AppSettings(
  customizationSettings: json['customizationSettings'] == null
      ? null
      : CustomizationSettings.fromJson(
          json['customizationSettings'] as Map<String, dynamic>,
        ),
  armaSettings: json['armaSettings'] == null
      ? null
      : ArmaSettings.fromJson(json['armaSettings'] as Map<String, dynamic>),
);

Map<String, dynamic> _$AppSettingsToJson(AppSettings instance) =>
    <String, dynamic>{
      'customizationSettings': instance.customizationSettings.toJson(),
      'armaSettings': instance.armaSettings.toJson(),
    };
