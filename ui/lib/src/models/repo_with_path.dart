import 'package:pamm_ui/src/rust/api/commands/init_from_remote.dart';
import 'package:pamm_ui/src/rust/api/commands/user_repo_settings.dart';

class RepoWithPath {
  final RepoConfig repo;
  final FlutterRepoUserSettings settings;
  final String path;

  RepoWithPath(this.repo, this.settings, this.path);
}
