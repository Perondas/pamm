// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'arma_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ArmaSettings _$ArmaSettingsFromJson(Map<String, dynamic> json) => ArmaSettings(
  launchType:
      $enumDecodeNullable(_$LaunchTypeEnumMap, json['launchType']) ??
      LaunchType.file,
);

Map<String, dynamic> _$ArmaSettingsToJson(ArmaSettings instance) =>
    <String, dynamic>{'launchType': _$LaunchTypeEnumMap[instance.launchType]!};

const _$LaunchTypeEnumMap = {
  LaunchType.steam: 'steam',
  LaunchType.file: 'file',
};
