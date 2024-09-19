build_project:
	cargo build

build_project_release:
	cargo build --release --all-features

build_wasm:
	cd wasm; wasm-pack build --target bundler

build_wasm_release:
	cd wasm; wasm-pack build --target bundler --release

build_ui:
	cd wasm/toolboxrs-ui; npm run build

build_all: build_project build_wasm build_ui
	

build_all_release: build_project_release build_wasm_release build_ui
	
