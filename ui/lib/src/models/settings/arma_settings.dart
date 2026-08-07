import 'package:json_annotation/json_annotation.dart';
import 'package:pamm_ui/src/rust/api/commands/launch.dart';

part 'arma_settings.g.dart';

@JsonSerializable()
class ArmaSettings {
  ArmaSettings({this.launchType = LaunchType.steam});

  LaunchType launchType;

  factory ArmaSettings.fromJson(Map<String, dynamic> json) =>
      _$ArmaSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$ArmaSettingsToJson(this);
}
