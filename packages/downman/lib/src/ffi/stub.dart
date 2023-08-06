import 'package:downman/src/bridge_generated.dart';

/// Represents the external library for downman
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

Downman createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();

Object createLibraryImpl() => throw UnimplementedError();
