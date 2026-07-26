import 'dart:io';

import 'package:flutter/material.dart';
import 'package:pamm_ui/src/util/media.dart';

/// List-tile leading icon backed by a repo media file. The image is drawn
/// as-is (no background fill, no circle crop) so transparency is respected;
/// without a usable file it falls back to a letter avatar.
class MediaIcon extends StatelessWidget {
  const MediaIcon({required this.iconFile, required this.fallback, super.key});

  final File? iconFile;
  final String fallback;

  @override
  Widget build(BuildContext context) {
    final letterAvatar = CircleAvatar(
      child: Text(fallback.isNotEmpty ? fallback[0].toUpperCase() : '?'),
    );

    final file = iconFile;
    if (file == null) return letterAvatar;

    return SizedBox(
      width: 40,
      height: 40,
      child: Image(
        image: MediaFileImage(file),
        fit: BoxFit.contain,
        errorBuilder: (_, _, _) => letterAvatar,
      ),
    );
  }
}
