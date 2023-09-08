.DEFAULT_GOAL := build

.PHONY: web desktop

all: build install

watch:
	cargo watch -- make run

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets --all-features -- -D warnings

build-wasm:
	cargo build --release --target wasm32-unknown-unknown
	wasm-opt -O4 ./target/wasm32-unknown-unknown/release/lr35902.wasm -o gameboy.wasm && du -h gameboy.wasm

test:
	cargo test --release

desktop:
	cd examples/desktop && cargo run

web-build-dev:
	cd docs && make build-dev

web-publish:
	cd docs && make publish

web-build:
	cd docs && make build

web:
	cd docs && make local
