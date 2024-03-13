
alias c := clean
alias b := build
alias r := run

clean:
	cargo clean
build: clean
	cargo build
run: build
	cargo run