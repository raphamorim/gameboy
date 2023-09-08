.DEFAULT_GOAL := build

.PHONY: web desktop

all: build install

watch:
	cargo watch -- make run

lint:
	cargo fmt --check

build-wasm:
	cargo build --release --target wasm32-unknown-unknown
	wasm-opt -O4 ./target/wasm32-unknown-unknown/release/lr35902.wasm -o gameboy.wasm && du -h gameboy.wasm

test:
	cargo test --release

desktop:
	cd ./examples/window && RUST_BACKTRACE=1 make dl

web-build-dev:
	cd website && make build-dev

web-publish:
	cd website && make publish

web-build:
	cd website && make build

web:
	cd website && make local
