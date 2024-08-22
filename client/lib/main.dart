import 'package:client/service/rust_service.dart';
import 'package:client/widgets/app.dart';
import 'package:flutter/material.dart';
import 'package:client/src/rust/frb_generated.dart';


Future<void> main() async {  
  await RustLib.init();
  await RustService.init();
  // await RustService.init();

  runApp(const App());
}
