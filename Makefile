.PHONY: help clean coverage dev integrate lint package package-install pypi-install test upload uninstall

integrate:
	cargo run mpersist > /tmp/mpersist.stdout
	diff /tmp/mpersist.stdout integrate/mpersist.stdout

test:
	cargo test
