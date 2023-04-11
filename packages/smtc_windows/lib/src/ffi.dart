import 'package:smtc_windows/src/bridge_generated.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

SmtcWindows? _wrapper;

SmtcWindows createLib() {
  _wrapper ??= createWrapperImpl(createLibraryImpl());
  return _wrapper!;
}