.PHONY: *
export RUST_BACKTRACE=1

dev:
	bun run tauri dev

test:
	cd src-tauri && cargo test

setup:
	bun install
	cd src-tauri && cargo build

build:
	bun run tauri build --no-bundle

open_release:
	open src-tauri/target/release

build_dmg:
	bun run tauri build

open_app_folder:
	open ~/Library/Application\ Support/io.kbl.alic

release:
	bun run scripts/release.ts
