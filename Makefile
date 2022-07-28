.DEFAULT_GOAL := build

.PHONY: web desktop

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

############################
######## Desktop ###########
############################
br:
	make build-desktop && make run-desktop

desktop-build:
	cd desktop && rustup run nightly cargo build

desktop:
#	Runs with a demo
	./desktop/target/debug/desktop ./tests/cpu_instrs/cpu_instrs.gb

############################
######## LR35902 ###########
############################
build:
	wasm-pack build --out-dir wasm --debug

############################
######### Server ###########
############################
web-build:
	cd web && rustup run nightly cargo build --release && ./target/release/web

web:
	yarn && yarn serve

# TODO: migrate to Rust
web-rust:
	open http://0.0.0.0:3000; cd web && ./target/release/web