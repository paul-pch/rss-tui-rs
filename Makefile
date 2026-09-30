.DEFAULT_GOAL := help

.PHONY: all fmt lint test help

all: fmt lint test

fmt:
	@echo "-> Formatting.."
	cargo fmt

fmt-check:
	@echo "-> Checking format.."
	cargo fmt --check

lint:
	@echo "-> Checking lint.."
	cargo clippy --all-targets -- -D warnings

test:
	@echo "-> Checking tests.."
	cargo test

help:
	@echo "Available targets :"
	@echo "  all      - Run "
	@echo "  fmt      - Format the code"
	@echo "  clippy   - Static analysis"
	@echo "  test     - Run all tests"
	@echo "  help     - Display help/usage"
