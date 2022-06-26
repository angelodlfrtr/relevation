.PHONY: lint
lint:
	cargo clippy

.PHONY: build
build:
	cargo build --release

.PHONY: build-upx
build-upx:
	cargo build --release
	upx --best target/release/relevation
