import 'dart:ui';

import 'package:json_annotation/json_annotation.dart';

part 'customization_settings.g.dart';

@JsonSerializable()
class CustomizationSettings {
  CustomizationSettings({this.seedColor, this.fixedSeedColor = false});

  @ColorJsonConverter()
  Color? seedColor;

  bool fixedSeedColor;

  factory CustomizationSettings.fromJson(Map<String, dynamic> json) =>
      _$CustomizationSettingsFromJson(json);

  Map<String, dynamic> toJson() => _$CustomizationSettingsToJson(this);
}

class ColorJsonConverter extends JsonConverter<Color, int> {
  const ColorJsonConverter();

  @override
  Color fromJson(int json) => Color(json);

  @override
  int toJson(Color object) => object.toARGB32();
}
