default: serve

build:
	cargo build --release --target=wasm32-unknown-unknown

build-bindgen: build
	wasm-bindgen target/wasm32-unknown-unknown/release/quark.wasm --out-dir www --target web --no-typescript

serve: build-bindgen
	python -m http.server -d www

setup:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli
