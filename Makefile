.DEFAULT_GOAL := help

.PHONY: all fmt fmt-check lint test help

all: fmt-check lint test

fmt:
	@echo "-> Checking format.."
	cargo fmt --check

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
	@echo "  all      	- Run "
	@echo "  fmt		- Format the code"
	@echo "  fmt-check	- Verify code format"
	@echo "  lint   	- Static analysis"
	@echo "  test     	- Run all tests"
	@echo "  help     	- Display help/usage"
