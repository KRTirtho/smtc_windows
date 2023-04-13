// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.72.2.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';

class SmtcWindowsPlatform extends FlutterRustBridgeBase<SmtcWindowsWire> with FlutterRustBridgeSetupMixin {
  SmtcWindowsPlatform(FutureOr<WasmModule> dylib) : super(SmtcWindowsWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  Object api2wire_SmtcInternal(SmtcInternal raw) {
    return raw.shareOrMove();
  }

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  Object api2wire_box_autoadd_i64(int raw) {
    return api2wire_i64(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_music_metadata(MusicMetadata raw) {
    return api2wire_music_metadata(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_playback_timeline(PlaybackTimeline raw) {
    return api2wire_playback_timeline(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_smtc_config(SMTCConfig raw) {
    return api2wire_smtc_config(raw);
  }

  @protected
  Object api2wire_i64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  List<dynamic> api2wire_music_metadata(MusicMetadata raw) {
    return [
      api2wire_String(raw.title),
      api2wire_String(raw.artist),
      api2wire_String(raw.album),
      api2wire_String(raw.albumArtist),
      api2wire_u32(raw.trackNumber),
      api2wire_opt_String(raw.thumbnail)
    ];
  }

  @protected
  String? api2wire_opt_String(String? raw) {
    return raw == null ? null : api2wire_String(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_i64(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_i64(raw);
  }

  @protected
  List<dynamic> api2wire_playback_timeline(PlaybackTimeline raw) {
    return [
      api2wire_i64(raw.startTimeMs),
      api2wire_i64(raw.endTimeMs),
      api2wire_i64(raw.positionMs),
      api2wire_opt_box_autoadd_i64(raw.minSeekTimeMs),
      api2wire_opt_box_autoadd_i64(raw.maxSeekTimeMs)
    ];
  }

  @protected
  List<dynamic> api2wire_smtc_config(SMTCConfig raw) {
    return [
      api2wire_bool(raw.playEnabled),
      api2wire_bool(raw.pauseEnabled),
      api2wire_bool(raw.stopEnabled),
      api2wire_bool(raw.nextEnabled),
      api2wire_bool(raw.prevEnabled),
      api2wire_bool(raw.fastForwardEnabled),
      api2wire_bool(raw.rewindEnabled)
    ];
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer

  late final Finalizer<PlatformPointer> _SmtcInternalFinalizer = Finalizer<PlatformPointer>(inner.drop_opaque_SmtcInternal);
  Finalizer<PlatformPointer> get SmtcInternalFinalizer => _SmtcInternalFinalizer;
}

// Section: WASM wire module

@JS('wasm_bindgen')
external SmtcWindowsWasmModule get wasmModule;

@JS()
@anonymous
class SmtcWindowsWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external SmtcWindowsWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* Object */ wire_smtc_new();

  external dynamic /* void */ wire_smtc_update_config(NativePortType port_, Object internal, List<dynamic> config);

  external dynamic /* void */ wire_smtc_update_metadata(NativePortType port_, Object internal, List<dynamic> metadata);

  external dynamic /* void */ wire_smtc_update_timeline(NativePortType port_, Object internal, List<dynamic> timeline);

  external dynamic /* void */ wire_smtc_update_playback_status(NativePortType port_, Object internal, int status);

  external dynamic /* void */ wire_smtc_update_shuffle(NativePortType port_, Object internal, bool shuffle);

  external dynamic /* void */ wire_smtc_update_repeat_mode(NativePortType port_, Object internal, String repeat_mode);

  external dynamic /* void */ wire_smtc_disable_smtc(NativePortType port_, Object internal);

  external dynamic /* void */ wire_smtc_button_press_event(NativePortType port_, Object internal);

  external dynamic /* void */ wire_smtc_position_change_request_event(NativePortType port_, Object internal);

  external dynamic /* void */ wire_smtc_shuffle_request_event(NativePortType port_, Object internal);

  external dynamic /* void */ wire_smtc_repeat_mode_request_event(NativePortType port_, Object internal);

  external dynamic /*  */ drop_opaque_SmtcInternal(ptr);

  external int /* *const c_void */ share_opaque_SmtcInternal(ptr);
}

// Section: WASM wire connector

class SmtcWindowsWire extends FlutterRustBridgeWasmWireBase<SmtcWindowsWasmModule> {
  SmtcWindowsWire(FutureOr<WasmModule> module) : super(WasmModule.cast<SmtcWindowsWasmModule>(module));

  dynamic /* Object */ wire_smtc_new() => wasmModule.wire_smtc_new();

  void wire_smtc_update_config(NativePortType port_, Object internal, List<dynamic> config) => wasmModule.wire_smtc_update_config(port_, internal, config);

  void wire_smtc_update_metadata(NativePortType port_, Object internal, List<dynamic> metadata) => wasmModule.wire_smtc_update_metadata(port_, internal, metadata);

  void wire_smtc_update_timeline(NativePortType port_, Object internal, List<dynamic> timeline) => wasmModule.wire_smtc_update_timeline(port_, internal, timeline);

  void wire_smtc_update_playback_status(NativePortType port_, Object internal, int status) => wasmModule.wire_smtc_update_playback_status(port_, internal, status);

  void wire_smtc_update_shuffle(NativePortType port_, Object internal, bool shuffle) => wasmModule.wire_smtc_update_shuffle(port_, internal, shuffle);

  void wire_smtc_update_repeat_mode(NativePortType port_, Object internal, String repeat_mode) => wasmModule.wire_smtc_update_repeat_mode(port_, internal, repeat_mode);

  void wire_smtc_disable_smtc(NativePortType port_, Object internal) => wasmModule.wire_smtc_disable_smtc(port_, internal);

  void wire_smtc_button_press_event(NativePortType port_, Object internal) => wasmModule.wire_smtc_button_press_event(port_, internal);

  void wire_smtc_position_change_request_event(NativePortType port_, Object internal) => wasmModule.wire_smtc_position_change_request_event(port_, internal);

  void wire_smtc_shuffle_request_event(NativePortType port_, Object internal) => wasmModule.wire_smtc_shuffle_request_event(port_, internal);

  void wire_smtc_repeat_mode_request_event(NativePortType port_, Object internal) => wasmModule.wire_smtc_repeat_mode_request_event(port_, internal);

  dynamic /*  */ drop_opaque_SmtcInternal(ptr) => wasmModule.drop_opaque_SmtcInternal(ptr);

  int /* *const c_void */ share_opaque_SmtcInternal(ptr) => wasmModule.share_opaque_SmtcInternal(ptr);
}
