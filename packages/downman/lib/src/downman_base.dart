import 'package:downman/src/ffi.dart';

import 'bridge_generated.dart' hide Downman;
import 'bridge_generated.dart' as impl;

impl.Downman? _api = null;

impl.Downman get api {
  if (_api == null) {
    throw Exception('Downman not initialized');
  }
  return _api!;
}

class Downman {
  static void initialize() {
    if (_api != null) {
      throw Exception('Downman already initialized');
    }
    _api = createLib();
  }

  final HttpClient _client;

  Downman([BaseConfig? config]) : _client = api.httpClientNew(config: config);

  Future<HttpResponse> get(
    String url, {
    Config? config,
  }) {
    return api.httpClientGet(
      client: _client,
      url: url,
      config: config,
    );
  }
}
