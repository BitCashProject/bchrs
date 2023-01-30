import 'dart:ffi';

import "../pkg/bridge_generated.dart" as rust;

Future<void> main(List<String> arguments) async {
  final dylib = DynamicLibrary.open("target/debug/dart.dll");
  final api = rust.DartImpl(dylib);
  final result = await api.add(left: 255, right: 10);

  print('Hello world: $result!');
}
