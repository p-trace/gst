.PHONY: all build

all: build

build-linux-static:
	@echo "Building (static linked) for x86_64 Linux"
	@echo "Note: If it fails, you'll need to install musl:"
	@echo "      `rustup target add x86_64-unknown-linux-musl`"
	cargo build --release --target x86_64-unknown-linux-musl

build:
	@echo "Building for current OS"
	cargo build --release

