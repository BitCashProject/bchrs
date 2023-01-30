flutter_rust_bridge_codegen --rust-input src/lib.rs --dart-output pkg/bridge_generated.dart
cargo build
dart bin/example.dart
