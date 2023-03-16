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
	cd ./examples/desktop && make desktop-build && RUST_BACKTRACE=1 make desktop

web-build:
	yarn && wasm-pack build --debug

web-publish:
	yarn && wasm-pack build --release
	du -k ./pkg
	npm publish

web-docs:
	yarn webpack && du -h ./docs

web:
	yarn serve
