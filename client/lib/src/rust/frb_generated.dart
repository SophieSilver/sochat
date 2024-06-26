// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api/service.dart';
import 'api/types/id.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.js_interop) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {
    await api.crateApiInitInitApp();
  }

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  @override
  String get codegenVersion => '2.0.0';

  @override
  int get rustContentHash => -316241126;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'rust_lib_client',
    ioDirectory: '../client_frb/target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  Future<void> crateApiInitInitApp();

  String? crateApiServiceServiceGetMessage(
      {required Service that,
      required UserId from,
      required UserId to,
      required PlatformInt64 index});

  PlatformInt64 crateApiServiceServiceMessageCount(
      {required Service that, required UserId from, required UserId to});

  Service crateApiServiceServiceNew();

  void crateApiServiceServiceSendMessage(
      {required Service that,
      required UserId from,
      required UserId to,
      required String message});

  bool crateApiTypesIdMessageIdEquals(
      {required MessageId that, required MessageId other});

  PlatformInt64 crateApiTypesIdMessageIdHashCode({required MessageId that});

  MessageId crateApiTypesIdMessageIdParse({required String value});

  String crateApiTypesIdMessageIdToStringDart({required MessageId that});

  bool crateApiTypesIdUserIdEquals(
      {required UserId that, required UserId other});

  PlatformInt64 crateApiTypesIdUserIdHashCode({required UserId that});

  UserId crateApiTypesIdUserIdParse({required String value});

  String crateApiTypesIdUserIdToStringDart({required UserId that});

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_MessageId;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_MessageId;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_MessageIdPtr;

  RustArcIncrementStrongCountFnType get rust_arc_increment_strong_count_Service;

  RustArcDecrementStrongCountFnType get rust_arc_decrement_strong_count_Service;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ServicePtr;

  RustArcIncrementStrongCountFnType get rust_arc_increment_strong_count_UserId;

  RustArcDecrementStrongCountFnType get rust_arc_decrement_strong_count_UserId;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_UserIdPtr;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<void> crateApiInitInitApp() {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire.wire__crate__api__init__init_app(port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiInitInitAppConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiInitInitAppConstMeta => const TaskConstMeta(
        debugName: "init_app",
        argNames: [],
      );

  @override
  String? crateApiServiceServiceGetMessage(
      {required Service that,
      required UserId from,
      required UserId to,
      required PlatformInt64 index}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
                that);
        var arg1 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                from);
        var arg2 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                to);
        var arg3 = cst_encode_i_64(index);
        return wire.wire__crate__api__service__Service_get_message(
            arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_opt_String,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiServiceServiceGetMessageConstMeta,
      argValues: [that, from, to, index],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiServiceServiceGetMessageConstMeta =>
      const TaskConstMeta(
        debugName: "Service_get_message",
        argNames: ["that", "from", "to", "index"],
      );

  @override
  PlatformInt64 crateApiServiceServiceMessageCount(
      {required Service that, required UserId from, required UserId to}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
                that);
        var arg1 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                from);
        var arg2 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                to);
        return wire.wire__crate__api__service__Service_message_count(
            arg0, arg1, arg2);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_i_64,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiServiceServiceMessageCountConstMeta,
      argValues: [that, from, to],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiServiceServiceMessageCountConstMeta =>
      const TaskConstMeta(
        debugName: "Service_message_count",
        argNames: ["that", "from", "to"],
      );

  @override
  Service crateApiServiceServiceNew() {
    return handler.executeSync(SyncTask(
      callFfi: () {
        return wire.wire__crate__api__service__Service_new();
      },
      codec: DcoCodec(
        decodeSuccessData:
            dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiServiceServiceNewConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiServiceServiceNewConstMeta => const TaskConstMeta(
        debugName: "Service_new",
        argNames: [],
      );

  @override
  void crateApiServiceServiceSendMessage(
      {required Service that,
      required UserId from,
      required UserId to,
      required String message}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
                that);
        var arg1 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                from);
        var arg2 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                to);
        var arg3 = cst_encode_String(message);
        return wire.wire__crate__api__service__Service_send_message(
            arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiServiceServiceSendMessageConstMeta,
      argValues: [that, from, to, message],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiServiceServiceSendMessageConstMeta =>
      const TaskConstMeta(
        debugName: "Service_send_message",
        argNames: ["that", "from", "to", "message"],
      );

  @override
  bool crateApiTypesIdMessageIdEquals(
      {required MessageId that, required MessageId other}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
                that);
        var arg1 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
                other);
        return wire.wire__crate__api__types__id__MessageId_equals(arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_bool,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdMessageIdEqualsConstMeta,
      argValues: [that, other],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdMessageIdEqualsConstMeta =>
      const TaskConstMeta(
        debugName: "MessageId_equals",
        argNames: ["that", "other"],
      );

  @override
  PlatformInt64 crateApiTypesIdMessageIdHashCode({required MessageId that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
                that);
        return wire.wire__crate__api__types__id__MessageId_hash_code(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_i_64,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdMessageIdHashCodeConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdMessageIdHashCodeConstMeta =>
      const TaskConstMeta(
        debugName: "MessageId_hash_code",
        argNames: ["that"],
      );

  @override
  MessageId crateApiTypesIdMessageIdParse({required String value}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 = cst_encode_String(value);
        return wire.wire__crate__api__types__id__MessageId_parse(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData:
            dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiTypesIdMessageIdParseConstMeta,
      argValues: [value],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdMessageIdParseConstMeta =>
      const TaskConstMeta(
        debugName: "MessageId_parse",
        argNames: ["value"],
      );

  @override
  String crateApiTypesIdMessageIdToStringDart({required MessageId that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
                that);
        return wire.wire__crate__api__types__id__MessageId_to_string_dart(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_String,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdMessageIdToStringDartConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdMessageIdToStringDartConstMeta =>
      const TaskConstMeta(
        debugName: "MessageId_to_string_dart",
        argNames: ["that"],
      );

  @override
  bool crateApiTypesIdUserIdEquals(
      {required UserId that, required UserId other}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                that);
        var arg1 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                other);
        return wire.wire__crate__api__types__id__UserId_equals(arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_bool,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdUserIdEqualsConstMeta,
      argValues: [that, other],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdUserIdEqualsConstMeta =>
      const TaskConstMeta(
        debugName: "UserId_equals",
        argNames: ["that", "other"],
      );

  @override
  PlatformInt64 crateApiTypesIdUserIdHashCode({required UserId that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                that);
        return wire.wire__crate__api__types__id__UserId_hash_code(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_i_64,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdUserIdHashCodeConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdUserIdHashCodeConstMeta =>
      const TaskConstMeta(
        debugName: "UserId_hash_code",
        argNames: ["that"],
      );

  @override
  UserId crateApiTypesIdUserIdParse({required String value}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 = cst_encode_String(value);
        return wire.wire__crate__api__types__id__UserId_parse(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData:
            dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiTypesIdUserIdParseConstMeta,
      argValues: [value],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdUserIdParseConstMeta => const TaskConstMeta(
        debugName: "UserId_parse",
        argNames: ["value"],
      );

  @override
  String crateApiTypesIdUserIdToStringDart({required UserId that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        var arg0 =
            cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
                that);
        return wire.wire__crate__api__types__id__UserId_to_string_dart(arg0);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_String,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTypesIdUserIdToStringDartConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTypesIdUserIdToStringDartConstMeta =>
      const TaskConstMeta(
        debugName: "UserId_to_string_dart",
        argNames: ["that"],
      );

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_MessageId => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_MessageId => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_Service => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_Service => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_UserId => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_UserId => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return AnyhowException(raw as String);
  }

  @protected
  MessageId
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return MessageIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  Service
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ServiceImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  UserId
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return UserIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  MessageId
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return MessageIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  Service
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ServiceImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  UserId
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return UserIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  MessageId
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return MessageIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  Service
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ServiceImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  UserId
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return UserIdImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  String dco_decode_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as String;
  }

  @protected
  IdExt dco_decode_TraitDef_IdExt(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  bool dco_decode_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as bool;
  }

  @protected
  PlatformInt64 dco_decode_i_64(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeI64(raw);
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as Uint8List;
  }

  @protected
  String? dco_decode_opt_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_String(raw);
  }

  @protected
  int dco_decode_u_8(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return;
  }

  @protected
  BigInt dco_decode_usize(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeU64(raw);
  }

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_String(deserializer);
    return AnyhowException(inner);
  }

  @protected
  MessageId
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return MessageIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  Service
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ServiceImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  UserId
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return UserIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  MessageId
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return MessageIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  Service
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ServiceImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  UserId
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return UserIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  MessageId
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return MessageIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  Service
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ServiceImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  UserId
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return UserIdImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  String sse_decode_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return utf8.decoder.convert(inner);
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  PlatformInt64 sse_decode_i_64(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getPlatformInt64();
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_String(deserializer));
    } else {
      return null;
    }
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getBigUint64();
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getInt32();
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
      MessageId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as MessageIdImpl).frbInternalCstEncode(move: true);
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
      Service raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as ServiceImpl).frbInternalCstEncode(move: true);
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
      UserId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as UserIdImpl).frbInternalCstEncode(move: true);
  }

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
      MessageId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as MessageIdImpl).frbInternalCstEncode(move: false);
  }

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
      Service raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as ServiceImpl).frbInternalCstEncode(move: false);
  }

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
      UserId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as UserIdImpl).frbInternalCstEncode(move: false);
  }

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
      MessageId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as MessageIdImpl).frbInternalCstEncode();
  }

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
      Service raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as ServiceImpl).frbInternalCstEncode();
  }

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
      UserId raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
// ignore: invalid_use_of_internal_member
    return (raw as UserIdImpl).frbInternalCstEncode();
  }

  @protected
  bool cst_encode_bool(bool raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  int cst_encode_u_8(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  void cst_encode_unit(void raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(self.message, serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          MessageId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as MessageIdImpl).frbInternalSseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          Service self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ServiceImpl).frbInternalSseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          UserId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as UserIdImpl).frbInternalSseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          MessageId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as MessageIdImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          Service self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ServiceImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          UserId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as UserIdImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMessageId(
          MessageId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as MessageIdImpl).frbInternalSseEncode(move: null), serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerService(
          Service self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ServiceImpl).frbInternalSseEncode(move: null), serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerUserId(
          UserId self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as UserIdImpl).frbInternalSseEncode(move: null), serializer);
  }

  @protected
  void sse_encode_String(String self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(utf8.encoder.convert(self), serializer);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self ? 1 : 0);
  }

  @protected
  void sse_encode_i_64(PlatformInt64 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putPlatformInt64(self);
  }

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(self);
  }

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_String(self, serializer);
    }
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putBigUint64(self);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putInt32(self);
  }
}

@sealed
class MessageIdImpl extends RustOpaque implements MessageId {
  // Not to be used by end users
  MessageIdImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  MessageIdImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_MessageId,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_MessageId,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_MessageIdPtr,
  );

  /// Use instead of == operator due to FRB limitations
  bool equals(MessageId other) => RustLib.instance.api
      .crateApiTypesIdMessageIdEquals(that: this, other: other);

  PlatformInt64 get hashCode =>
      RustLib.instance.api.crateApiTypesIdMessageIdHashCode(
        that: this,
      );

  String toString() =>
      RustLib.instance.api.crateApiTypesIdMessageIdToStringDart(
        that: this,
      );
}

@sealed
class ServiceImpl extends RustOpaque implements Service {
  // Not to be used by end users
  ServiceImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  ServiceImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Service,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Service,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_ServicePtr,
  );

  String? getMessage(
          {required UserId from,
          required UserId to,
          required PlatformInt64 index}) =>
      RustLib.instance.api.crateApiServiceServiceGetMessage(
          that: this, from: from, to: to, index: index);

  PlatformInt64 messageCount({required UserId from, required UserId to}) =>
      RustLib.instance.api
          .crateApiServiceServiceMessageCount(that: this, from: from, to: to);

  void sendMessage(
          {required UserId from,
          required UserId to,
          required String message}) =>
      RustLib.instance.api.crateApiServiceServiceSendMessage(
          that: this, from: from, to: to, message: message);
}

@sealed
class UserIdImpl extends RustOpaque implements UserId {
  // Not to be used by end users
  UserIdImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  UserIdImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_UserId,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_UserId,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_UserIdPtr,
  );

  /// Use instead of == operator due to FRB limitations
  bool equals(UserId other) => RustLib.instance.api
      .crateApiTypesIdUserIdEquals(that: this, other: other);

  PlatformInt64 get hashCode =>
      RustLib.instance.api.crateApiTypesIdUserIdHashCode(
        that: this,
      );

  String toString() => RustLib.instance.api.crateApiTypesIdUserIdToStringDart(
        that: this,
      );
}
