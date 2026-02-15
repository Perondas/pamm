import 'dart:convert';

import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

class RepoPathStore {
  static const _key = 'pamm_repositories_v2';

  // Load all stored repos
  static Future<Set<String>> getRepoPaths() async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    final list = prefs.getStringList(_key);
    if (list == null) return <String>{};
    return list.toSet();
  }

  static Future<void> _saveAll(Set<String> paths) async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    await prefs.setStringList(_key, paths.toList(growable: false));
  }

  static Future<void> add(String repoPath) async {
    final set = await getRepoPaths();
    if (set.add(repoPath)) {
      await _saveAll(set);
    }
  }

  static Future<void> remove(String repoPath) async {
    final set = await getRepoPaths();
    if (set.remove(repoPath)) {
      await _saveAll(set);
    }
  }

  static Future<void> clear() async {
    final prefs = await GetIt.instance.getAsync<SharedPreferencesWithCache>();
    await prefs.remove(_key);
  }
}
