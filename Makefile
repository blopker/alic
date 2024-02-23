build: build_rust
	flutter build macos

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