import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:pamm_ui/src/pages/main_screen/main.dart';
import 'package:pamm_ui/src/rust/frb_generated.dart';
import 'package:pamm_ui/src/services/rust_log_service.dart';
import 'package:pamm_ui/src/services/settings_service.dart';
import 'package:pamm_ui/src/services/update_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

final getIt = GetIt.instance;
late final RustLogService rustLogService;

const defaultSeedColor = Color.fromARGB(255, 236, 214, 153);

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  configureDI();
  await getIt.allReady();
  await settingsService.load();

  rustLogService = RustLogService();

  Future.delayed(Duration(seconds: 2), () {
    checkForUpdates();
  });

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: settingsService,
      builder: (context, _) {
        return MaterialApp(
          navigatorKey: NavigationService.navigatorKey,
          home: MainScreen(),
          theme: ThemeData.from(
            colorScheme: ColorScheme.fromSeed(
              seedColor:
                  settingsService.settings.customizationSettings.seedColor ??
                  defaultSeedColor,
            ),
          ),
        );
      },
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


class NavigationService {
  static final navigatorKey = GlobalKey<NavigatorState>();
}