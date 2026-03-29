import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:pamm_ui/src/pages/main_screen/main.dart';
import 'package:pamm_ui/src/rust/frb_generated.dart';
import 'package:pamm_ui/src/services/rust_log_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

final getIt = GetIt.instance;
late final RustLogService rustLogService;

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  configureDI();
  await getIt.allReady();

  rustLogService = RustLogService();

  runApp(const MyApp());

  doWhenWindowReady(() {
    const initialSize = Size(600, 450);
    appWindow.minSize = initialSize;
    appWindow.alignment = Alignment.center;
    appWindow.show();
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: MainScreen(),
      theme: ThemeData.from(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Color.fromARGB(255, 236, 214, 153),
        ),
      ),
    );
  }
}

void configureDI() {
  getIt.registerSingletonAsync<SharedPreferencesWithCache>(() async {
    return await SharedPreferencesWithCache.create(
      cacheOptions: SharedPreferencesWithCacheOptions(),
    );
  });
}
