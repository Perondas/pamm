import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:ui/src/pages/main_screen/main.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: MainScreen(),
      theme: ThemeData.from(colorScheme: ColorScheme.light()),
    );
  }
}
