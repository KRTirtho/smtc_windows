import 'package:smtc_windows/src/bridge_generated.dart';

/// Represents the external library for smtc_windows
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

SmtcWindows createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();

Object createLibraryImpl() => throw UnimplementedError();
