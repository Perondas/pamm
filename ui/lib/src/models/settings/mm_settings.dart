import 'package:json_annotation/json_annotation.dart';

part 'mm_settings.g.dart';

@JsonSerializable()
class MmSettings {
  MmSettings({this.mmModeEnabled = false});

  bool mmModeEnabled;

  factory MmSettings.fromJson(Map<String, dynamic> json) =>
      _$MmSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$MmSettingsToJson(this);
}
