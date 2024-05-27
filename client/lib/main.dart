import 'package:client/src/rust/api/api.dart';
import 'package:client/src/rust/api/simple.dart';
import 'package:client/widgets/app.dart';
import 'package:flutter/material.dart';
import 'package:client/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const App());
  
  
}
