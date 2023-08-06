import 'dart:ffi';
import 'dart:io';

import 'package:downman/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

Downman createWrapperImpl(ExternalLibrary dylib) =>
    DownmanImpl(dylib);

DynamicLibrary createLibraryImpl() {
  const base = 'downman';

  if (Platform.isIOS || Platform.isMacOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open('$base.dll');
  } else {
    return DynamicLibrary.open('lib$base.so');
  }
}
