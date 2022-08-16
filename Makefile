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
######## Web ###############
############################
web-build:
	yarn && wasm-pack build --debug

web:
	yarn serve
