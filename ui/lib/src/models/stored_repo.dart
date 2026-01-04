import 'dart:convert';

import '../rust/api/commands/init_from_remote.dart';

class StoredRepo {
  final String name;
  final String description;
  final List<String> packs;
  final String path;

  StoredRepo({
    required this.name,
    required this.description,
    required this.packs,
    required this.path,
  });

  Map<String, dynamic> toJson() => {
        'name': name,
        'description': description,
        'packs': packs,
        'path': path,
      };

  factory StoredRepo.fromJson(Map<String, dynamic> json) => StoredRepo(
        name: json['name'] as String? ?? '',
        description: json['description'] as String? ?? '',
        packs: (json['packs'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
        path: json['path'] as String? ?? '',
      );

  // Convenience to build from the generated RepoConfig
  factory StoredRepo.fromRepoConfig(RepoConfig cfg, String path) => StoredRepo(
        name: cfg.name,
        description: cfg.description,
        packs: cfg.packs.toList(),
        path: path,
      );

  @override
  String toString() => jsonEncode(toJson());
}

