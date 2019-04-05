.PHONY: build-ramp dev integrate test

build-ramp:
	cargo +nightly build --no-default-features --features backend-ramp

dev:
	rustup component add rustfmt
	rustup toolchain install nightly

integrate:
	./integrate/check

test:
	cargo fmt -- --check
	./test/check
