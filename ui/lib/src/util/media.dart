import 'dart:io';

/// Resolves a media file name against a repo's `media/` directory.
/// Returns null when no name is set or the file does not exist (callers fall
/// back to the letter avatar / no banner).
File? mediaFile(String repoPath, String? name) {
  if (name == null || name.isEmpty) return null;
  final file = File(
    '$repoPath${Platform.pathSeparator}media${Platform.pathSeparator}$name',
  );
  return file.existsSync() ? file : null;
}
