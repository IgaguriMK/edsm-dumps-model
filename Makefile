CRATE_NAME:=edsm-dumps-model

.PHONY: all
all: check

.PHONY: check
check: soft-clean
	cargo test --all-features
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo audit

.PHONY: min-check
min-check:
	cargo +nightly update -Z minimal-versions
	cargo check --all-features

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean

.PHONY: download-dumps
download-dumps:
	make -C dumps download-dumps
