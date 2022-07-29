.DEFAULT_GOAL := build

.PHONY: web desktop

all: build install

# cargo install cargo-watch
watch:
	cargo watch -- make run

lint:
	cargo fmt --check

fix-lint:
	cargo fmt

test:
	cargo test --release

install:
	cargo install

start: build run

############################
######## Desktop ###########
############################
d:
	make desktop-build && make desktop

desktop-build:
	cd desktop && cargo build

desktop:
#	Runs with a demo
	./desktop/target/debug/desktop ./sample-rom.gb

############################
######## LR35902 ###########
############################
build:
	wasm-pack build --debug

############################
######### Server ###########
############################
web-build:
	cd web && cargo build --release && ./target/release/web

web:
	yarn && yarn serve

# TODO: migrate to Rust
web-rust:
	open http://0.0.0.0:3000; cd web && ./target/release/web