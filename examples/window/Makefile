############################
######## Desktop ###########
############################

.PHONY: desktop

# Debug
d:
	make desktop-build && make desktop

dl:
# 	rm ./log
	make desktop-build && make desktop >> log

desktop-build:
	cargo build

# Runs with a demo
desktop:
	./target/debug/window ./../../tests/cpu_instrs/cpu_instrs.gb