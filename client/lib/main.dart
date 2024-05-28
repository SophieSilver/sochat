import 'package:client/widgets/app.dart';
import 'package:flutter/material.dart';
import 'package:client/src/rust/frb_generated.dart';

Future<void> main() async {  
  await RustLib.init();
  runApp(const App());
}
