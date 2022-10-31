all:
	cargo build

day%:
	cargo test --bin $@
	cargo run --bin $@ < input/day$*.input

%:
	cargo build
