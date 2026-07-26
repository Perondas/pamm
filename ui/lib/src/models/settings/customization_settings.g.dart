// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customization_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomizationSettings _$CustomizationSettingsFromJson(
  Map<String, dynamic> json,
) => CustomizationSettings(
  seedColor: _$JsonConverterFromJson<int, Color>(
    json['seedColor'],
    const ColorJsonConverter().fromJson,
  ),
  fixedSeedColor: json['fixedSeedColor'] as bool? ?? false,
);

Map<String, dynamic> _$CustomizationSettingsToJson(
  CustomizationSettings instance,
) => <String, dynamic>{
  'seedColor': _$JsonConverterToJson<int, Color>(
    instance.seedColor,
    const ColorJsonConverter().toJson,
  ),
  'fixedSeedColor': instance.fixedSeedColor,
};

Value? _$JsonConverterFromJson<Json, Value>(
  Object? json,
  Value? Function(Json json) fromJson,
) => json == null ? null : fromJson(json as Json);

Json? _$JsonConverterToJson<Json, Value>(
  Value? value,
  Json? Function(Value value) toJson,
) => value == null ? null : toJson(value);
