import 'package:flutter/material.dart';

/// Marks [child] with a red download badge in its top-right corner when an
/// update is available. Uses fixed high-contrast colors (instead of the theme's
/// error color) so it stays visible with any seed color and on top of images.
class UpdateBadge extends StatelessWidget {
  const UpdateBadge({
    required this.visible,
    required this.child,
    this.size = 20,
    super.key,
  });

  final bool visible;
  final Widget child;

  /// Diameter of the badge.
  final double size;

  @override
  Widget build(BuildContext context) {
    if (!visible) return child;

    return Stack(
      clipBehavior: Clip.none,
      children: [
        child,
        Positioned(
          top: -size / 3,
          right: -size / 3,
          child: Tooltip(
            message: 'Updates available',
            child: Container(
              width: size,
              height: size,
              decoration: BoxDecoration(
                color: Colors.red.shade600,
                shape: BoxShape.circle,
              ),
              child: Icon(
                Icons.arrow_downward,
                size: size * 0.6,
                color: Colors.white,
              ),
            ),
          ),
        ),
      ],
    );
  }
}
