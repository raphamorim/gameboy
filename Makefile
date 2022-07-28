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

############################
######## Desktop ###########
############################
br:
	make build-desktop && make run-desktop

build-desktop:
	cd desktop && rustup run nightly cargo build

run-desktop:
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
build-server:
	cd server && rustup run nightly cargo build --release && ./target/release/server

run-server:
	./target/release/server