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
	RUST_LOG=debug wasm-pack build --out-dir wasm

bin:
#	My command to debug things quickly
	rustup run nightly cargo build && ./target/debug/LR35902 ./tests/cpu_instrs/cpu_instrs.gb

# build:
# 	rustup run nightly cargo build --release

run:
	./target/release/LF35902

build-debug:
	rustup run nightly cargo build

run-debug:
	./target/debug/LF35902

############################
######### Server ###########
############################
build-server:
	cd server && rustup run nightly cargo build --release && ./target/release/server

run-server:
	./target/release/server