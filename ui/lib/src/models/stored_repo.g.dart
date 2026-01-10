// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'stored_repo.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StoredRepo _$StoredRepoFromJson(Map<String, dynamic> json) => StoredRepo(
  name: json['name'] as String,
  description: json['description'] as String,
  packs: (json['packs'] as List<dynamic>).map((e) => e as String).toList(),
  path: json['path'] as String,
);

Map<String, dynamic> _$StoredRepoToJson(StoredRepo instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'packs': instance.packs,
      'path': instance.path,
    };
