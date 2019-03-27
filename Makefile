.PHONY: help clean coverage dev integrate lint package package-install pypi-install test upload uninstall

integrate:
	cargo run --bin mpersist -- search > /tmp/mpersist.stdout
	diff /tmp/mpersist.stdout integrate/mpersist.stdout

test:
	cargo fmt -- --check
	cargo test --locked
