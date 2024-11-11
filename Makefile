run:
	cargo run

run-with-args:
	cargo run -- $(ARGS)

test:
	cargo run -- test

help:
	cargo run -- help

help:
	cargo run -- help

fmt:
	cargo fmt --all

clippy:
	cargo clippy

clean:
	cargo clean

