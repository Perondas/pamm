import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:ui/src/pages/main_screen/main.dart';
import 'package:ui/src/rust/frb_generated.dart';
import 'package:ui/src/services/rust_log_service.dart';

final getIt = GetIt.instance;
late final RustLogService rustLogService;

Future<void> main() async {
  await RustLib.init();
  rustLogService = RustLogService();

  configureDI();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: MainScreen(),
      theme: ThemeData.from(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Color.fromARGB(100, 129, 107, 65),
        ),
      ),
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
