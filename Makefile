all: main

main: Cargo.toml
	cargo build --release

clean:
	rm -f -R ./target

dev: Cargo.toml
	cargo build
