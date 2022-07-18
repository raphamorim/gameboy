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

run:
	open index.html

install-wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

install:
	rustup run nightly cargo install

b:
	rustup run nightly cargo build --release

build:
	RUST_LOG=info wasm-pack build

# Debug
build-debug:
	rustup run nightly cargo build

start-debug:
	./target/debug/LR35902