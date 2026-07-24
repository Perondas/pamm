// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customization_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomizationSettings _$CustomizationSettingsFromJson(
  Map<String, dynamic> json,
) => CustomizationSettings()
  ..seedColor = _$JsonConverterFromJson<Map<String, int>, Color>(
    json['seedColor'],
    const ColorJsonConverter().fromJson,
  );

Map<String, dynamic> _$CustomizationSettingsToJson(
  CustomizationSettings instance,
) => <String, dynamic>{
  'seedColor': _$JsonConverterToJson<Map<String, int>, Color>(
    instance.seedColor,
    const ColorJsonConverter().toJson,
  ),
};

Value? _$JsonConverterFromJson<Json, Value>(
  Object? json,
  Value? Function(Json json) fromJson,
) => json == null ? null : fromJson(json as Json);

Json? _$JsonConverterToJson<Json, Value>(
  Value? value,
  Json? Function(Value value) toJson,
) => value == null ? null : toJson(value);
