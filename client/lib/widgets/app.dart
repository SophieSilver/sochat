import 'package:client/widgets/home.dart';
import 'package:flutter/material.dart';

const Color colorSchemeSeedColor = Color.fromARGB(255, 96, 10, 255);

class App extends StatefulWidget {
  const App({super.key});

  @override
  State<StatefulWidget> createState() {
    return _AppState();
  }
}

class _AppState extends State<App> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: "SoChat",
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: colorSchemeSeedColor,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
        applyElevationOverlayColor: true,
        fontFamily: "Noto Sans",
      ),
      home: Home(),
    );
  }
}
