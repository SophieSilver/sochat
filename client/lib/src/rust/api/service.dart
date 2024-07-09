// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'types/id.dart';

// These functions are ignored because they are not marked as `pub`: `inner`
// These types are ignored because they are not used by any `pub` functions: `ServiceInner`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`, `fmt`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `default`

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Service>>
abstract class Service implements RustOpaqueInterface {
  String? getMessage(
      {required UserId from, required UserId to, required PlatformInt64 index});

  PlatformInt64 messageCount({required UserId from, required UserId to});

  factory Service() => RustLib.instance.api.crateApiServiceServiceNew();

  void sendMessage(
      {required UserId from, required UserId to, required String message});
}
