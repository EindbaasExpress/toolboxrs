build_project:
	cargo build

build_wasm:
	cd wasm; wasm-pack build --target bundler

build_all: build_project build_wasm
