CRATE_NAME:=edsm-dumps-model

.PHONY: all
all: check

.PHONY: check
check: soft-clean
	cargo test
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo audit
	cargo outdated --exit-code 1

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean