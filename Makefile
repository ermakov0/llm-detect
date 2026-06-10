MAIN := $(shell basename $(PWD))
SOURCES := $(shell find src/ -name '*.rs')

all: target/x86_64-pc-windows-gnu/release/$(MAIN).exe

target/x86_64-pc-windows-gnu/release/%: Cargo.toml $(SOURCES)
	cargo build --target x86_64-pc-windows-gnu --release

clean:
	cargo clean
