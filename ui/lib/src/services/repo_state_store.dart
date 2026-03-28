import 'dart:async';

import 'package:flutter/cupertino.dart';
import 'package:pamm_ui/src/models/repo_with_path.dart';
import 'package:pamm_ui/src/rust/api/commands/load_repo.dart';
import 'package:pamm_ui/src/rust/api/commands/sync_config.dart';

class RepoStateManager with ChangeNotifier {
  final String repoPath;
  RepoWithPath? repoState;
  bool isConfigUpToDate = true;
  String? configLoadError;
  String? configUpdateError;

  bool get hasError => configLoadError != null || configUpdateError != null;

  bool get doneLoading => hasError || repoState != null;

  RepoStateManager(this.repoPath) {
    _loadRepoState();
  }

  Future<void> _loadRepoState() async {
    try {
      var repo = await loadRepo(repoPath: repoPath);
      repoState = RepoWithPath(repo, repoPath);
      notifyListeners();
    } catch (e) {
      repoState = null;
      configLoadError = "Failed to load repo at $repoPath: $e";
      notifyListeners();
      return;
    }

    _checkForConfigUpdates();
  }

  Future<void> _checkForConfigUpdates() async {
    try {
      var repo = await syncConfig(repoPath: repoPath);
      isConfigUpToDate = true;
      repoState = RepoWithPath(repo, repoPath);
      notifyListeners();
    } catch (e) {
      configUpdateError =
          "Failed to check for config updates for repo at $repoPath: $e";
      notifyListeners();
    }
  }
}
