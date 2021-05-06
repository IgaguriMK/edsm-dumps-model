CRATE_NAME:=edsm-dumps-model

.PHONY: all
all: check

.PHONY: check
check: soft-clean
	cargo test
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo audit

.PHONY: min-check
min-check:
	cargo +nightly update -Z minimal-versions
	cargo check

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean
