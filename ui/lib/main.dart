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
  await RustLib.init();
  rustLogService = RustLogService();

  configureDI();
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
      builder: (context, child) {
        return Scaffold(
          body: WindowBorder(
            color: Theme.of(context).colorScheme.primary,
            child: Column(
              children: [
                Material(
                  color: Theme.of(context).navigationDrawerTheme.backgroundColor ?? Theme.of(context).colorScheme.surfaceContainerLow,
                  child: WindowTitleBarBox(
                    child: Row(
                      children: [
                        Expanded(child: MoveWindow()),
                        const WindowButtons(),
                      ],
                    ),
                  ),
                ),
                Expanded(child: child ?? const SizedBox.shrink()),
              ],
            ),
          ),
        );
      },
      home: MainScreen(),
      theme: ThemeData.from(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF415AD9))
      ),
    );
  }
}

final buttonColors = WindowButtonColors(
    iconNormal: const Color(0xFF805306),
    mouseOver: const Color(0xFFF6A00C),
    mouseDown: const Color(0xFF805306),
    iconMouseOver: const Color(0xFF805306),
    iconMouseDown: const Color(0xFFFFD500));

final closeButtonColors = WindowButtonColors(
    mouseOver: const Color(0xFFD32F2F),
    mouseDown: const Color(0xFFB71C1C),
    iconNormal: const Color(0xFF805306),
    iconMouseOver: Colors.white);

class WindowButtons extends StatelessWidget {
  const WindowButtons({super.key});
  
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        MinimizeWindowButton(colors: buttonColors),
        MaximizeWindowButton(colors: buttonColors),
        CloseWindowButton(colors: closeButtonColors),
      ],
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
