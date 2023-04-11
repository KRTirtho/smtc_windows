import 'dart:ffi';
import 'dart:io';

import 'package:smtc_windows/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

SmtcWindows createWrapperImpl(ExternalLibrary dylib) =>
    SmtcWindowsImpl(dylib);

DynamicLibrary createLibraryImpl() {
  const base = 'smtc_windows';

  if (Platform.isIOS || Platform.isMacOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open('$base.dll');
  } else {
    return DynamicLibrary.open('lib$base.so');
  }
}
