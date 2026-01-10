import 'dart:convert';

import 'package:shared_preferences/shared_preferences.dart';

import '../models/stored_repo.dart';

class ReposStore {
  static const _key = 'pamm_repositories_v1';

  // Load all stored repos
  static Future<List<StoredRepo>> load() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = prefs.getStringList(_key);
    if (raw == null) return [];
    return raw.map((s) {
      try {
        final m = jsonDecode(s) as Map<String, dynamic>;
        return StoredRepo.from(m);
      } catch (e) {
        return StoredRepo(name: 'invalid', description: e.toString(), packs: [], path: '');
      }
    }).toList();
  }

  static Future<void> saveAll(List<StoredRepo> repos) async {
    final prefs = await SharedPreferences.getInstance();
    final encoded = repos.map((r) => jsonEncode(r.toJson())).toList();
    await prefs.setStringList(_key, encoded);
  }

  static Future<void> add(StoredRepo repo) async {
    final list = await load();
    list.add(repo);
    await saveAll(list);
  }

  static Future<void> remove(StoredRepo repo) async {
    final list = await load();
    list.removeWhere((r) => r.path == repo.path);
    await saveAll(list);
  }

  static Future<void> clear() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_key);
  }
}

