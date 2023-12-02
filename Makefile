run:
	cargo run -p game

build:
	cargo build -p game --release

clean:
	rm -rf ./target/

.PHONY: run build clean
