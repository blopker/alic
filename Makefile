build: build_rust
	flutter build macos

build_rust:
	flutter_rust_bridge_codegen generate

watch_rust:
	flutter_rust_bridge_codegen generate --watch

create_dmg:
	./scripts/create_dmg.sh

release:
	./scripts/release.sh