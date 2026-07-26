import 'dart:io';

import 'package:flutter/painting.dart';

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

/// [FileImage] whose cache key includes the file's last-modified time, so a
/// media file that changed on disk under the same name is reloaded instead of
/// being served stale from the image cache.
class MediaFileImage extends FileImage {
  MediaFileImage(super.file) : lastModified = _mtimeOrNull(file);

  final DateTime? lastModified;

  static DateTime? _mtimeOrNull(File file) {
    try {
      return file.lastModifiedSync();
    } on FileSystemException {
      return null;
    }
  }

  @override
  bool operator ==(Object other) =>
      other is MediaFileImage &&
      other.file.path == file.path &&
      other.scale == scale &&
      other.lastModified == lastModified;

  @override
  int get hashCode => Object.hash(file.path, scale, lastModified);
}
