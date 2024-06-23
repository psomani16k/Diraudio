import 'package:audio_library_convertor/ui/home_page/home_page.dart';
import 'package:dynamic_color/dynamic_color.dart';
import 'package:window_manager/window_manager.dart';

import './messages/generated.dart';
import 'package:flutter/material.dart';

void main() async {
  // set initial state of the app here
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();
  windowManager.setMinimumSize(const Size(500, 600));
  WindowManager.instance.setMinimumSize(const Size(500, 500));
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});
  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return DynamicColorBuilder(
      builder: (lightDynamic, darkDynamic) {
        return MaterialApp(
          theme: ThemeData(colorScheme: lightDynamic),
          darkTheme: ThemeData(colorScheme: darkDynamic),
          debugShowCheckedModeBanner: false,
          home: const ProgressUpdateProcessor(),
        );
      },
    );
  }
}
