all: main

main: Cargo.toml
	cargo build

clean:
	rm -f -R ./target