build: build_rust build_gen
	flutter build macos

watch:
	dart run build_runner watch --delete-conflicting-outputs

build_gen:
	dart run build_runner build --delete-conflicting-outputs

build_rust:
	flutter_rust_bridge_codegen generate

watch_rust:
	flutter_rust_bridge_codegen generate --watch

update_rust:
	cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0' && flutter_rust_bridge_codegen generate

create_dmg:
	./scripts/create_dmg.sh

release:
	./scripts/release.sh

run_release_build::
	open build/macos/Build/Products/Release/"Alic Image Compressor.app"