.DEFAULT_GOAL := build

.PHONY: web desktop

all: build install

# cargo install cargo-watch
watch:
	cargo watch -- make run

lint:
	cargo fmt --check
# 	cargo clippy --all-targets -- -D warnings

test:
	cargo test --release

############################
######## Desktop ###########
############################
desktop:
	cd ./examples/desktop && make desktop-build && RUST_BACKTRACE=1 make desktop

############################
######## LR35902 ###########
############################
w:
#	Debug
	make build

build:
	wasm-pack build --debug

run:
	yarn serve

############################
######### Server ###########
############################
web-server-build:
	cd web && cargo build --release && ./target/release/web

web-server-run:
	yarn && yarn serve

# TODO: migrate to Rust
web-rust:
	open http://0.0.0.0:3000; cd web && ./target/release/web