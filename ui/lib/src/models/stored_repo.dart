import 'package:json_annotation/json_annotation.dart';

import '../rust/api/commands/init_from_remote.dart';

part 'stored_repo.g.dart';

@JsonSerializable()
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

  // Convenience to build from the generated RepoConfig
  factory StoredRepo.fromRepoConfig(RepoConfig cfg, String path) => StoredRepo(
    name: cfg.name,
    description: cfg.description,
    packs: cfg.packs.toList(),
    path: path,
  );

  factory StoredRepo.from(Map<String, dynamic> json) =>
      _$StoredRepoFromJson(json);

  Map<String, dynamic> toJson() => _$StoredRepoToJson(this);
}
