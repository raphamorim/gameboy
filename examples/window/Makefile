############################
######## Desktop ###########
############################

.PHONY: desktop

# Debug
d:
	make desktop-build && RUST_BACKTRACE=1 make desktop

dl:
# 	rm ./log
	make desktop-build && RUST_BACKTRACE=1 make desktop >> log

desktop-build:
	cargo build

# Runs with a demo
desktop:
	./target/debug/desktop ./../../tests/cpu_instrs/cpu_instrs.gb