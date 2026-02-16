import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:ui/src/pages/main_screen/main.dart';
import 'package:ui/src/rust/api/commands/init_from_remote.dart';
import 'package:ui/src/rust/frb_generated.dart';

final getIt = GetIt.instance;

Future<void> main() async {
  await RustLib.init();
  configureDI();
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

void configureDI() {
  getIt.registerLazySingletonAsync<SharedPreferencesWithCache>(() async {
    return await SharedPreferencesWithCache.create(
      cacheOptions: SharedPreferencesWithCacheOptions(),
    );
  });
}
