.DEFAULT_GOAL := build

all: build install

# cargo install cargo-watch
watch:
	cargo watch -- make run

lint:
	rustup run nightly cargo fmt --check

fix-lint:
	rustup run nightly cargo fmt

test:
	rustup run nightly cargo test --release

install:
	rustup run nightly cargo install

start: build run

build:
# 	RUST_LOG=info wasm-pack build src/lib.rs --out-dir wasm
	rustup run nightly cargo build --release

run:
	./target/release/LF35902

build-debug:
	rustup run nightly cargo build

run-debug:
	./target/debug/LF35902