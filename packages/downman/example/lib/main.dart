import 'dart:convert';

import 'package:flutter/material.dart';
import 'dart:async';

import 'package:downman/downman.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  Downman.initialize();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late final Downman downman;

  @override
  void initState() {
    super.initState();
    downman = Downman();
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      final res =
          await downman.get("https://jsonplaceholder.typicode.com/todos/1");

      print(
        // bytes to utf text
        res.body != null
            ? jsonDecode(const Utf8Decoder().convert(res.body!.toList()))
            : "Failed to get body",
      );

      print(
        "Status code: ${res.status}\n"
        "Content headers: ${res.headers}\n",
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    const textStyle = TextStyle(fontSize: 25);
    const spacerSmall = SizedBox(height: 10);
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(10),
            child: const Column(
              children: [
                Text(
                  'This calls a native function through FFI that is shipped as source in the package. '
                  'The native code is built as part of the Flutter Runner build.',
                  style: textStyle,
                  textAlign: TextAlign.center,
                ),
                spacerSmall,
                spacerSmall,
              ],
            ),
          ),
        ),
      ),
    );
  }
}
