.PHONY: dev integrate test

dev:
	rustup component add rustfmt

integrate:
	cargo run --bin mpersist -- search > /tmp/mpersist.stdout
	diff /tmp/mpersist.stdout integrate/mpersist.stdout

test:
	cargo fmt -- --check
	cargo test --locked
