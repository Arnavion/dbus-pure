.PHONY: clean test

default: test

clean:
	rm -rf Cargo.lock target/

test:
	cargo test --all
	cargo clippy --all
