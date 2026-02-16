import 'dart:async';

import 'package:flutter/cupertino.dart';
import 'package:ui/src/models/repo_with_path.dart';
import 'package:ui/src/rust/api/commands/load_repo.dart';

class RepoStateManager with ChangeNotifier {
  final String repoPath;
  RepoWithPath? repoState;
  bool isConfigUpToDate = true;
  String? configLoadError;
  String? configUpdateError;

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
      //var isUpToDate = await checkRepoConfigUpToDate(repoPath: repoPath);
      var isUpToDate =
          true; // TODO: implement this in rust and uncomment above line
      isConfigUpToDate = isUpToDate;
      notifyListeners();
    } catch (e) {
      configUpdateError =
          "Failed to check for config updates for repo at $repoPath: $e";
      notifyListeners();
    }
  }
}
