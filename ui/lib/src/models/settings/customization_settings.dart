import 'dart:ui';

import 'package:json_annotation/json_annotation.dart';

part 'customization_settings.g.dart';

@JsonSerializable()
class CustomizationSettings {
  @ColorJsonConverter()
  Color? seedColor;
}

class ColorJsonConverter extends JsonConverter<Color, Map<String, int>> {
  const ColorJsonConverter();

  @override
  Color fromJson(Map<String, int> json) {
    return Color.fromARGB(
      json['a'] ?? 255,
      json['r'] ?? 0,
      json['g'] ?? 0,
      json['b'] ?? 0,
    );
  }

  @override
  Map<String, int> toJson(Color object) {
    return {
      'a': object.alpha,
      'r': object.red,
      'g': object.green,
      'b': object.blue,
    };
  }
}
