.PHONY: help build check test fmt clippy clean run docker-build docker-run docs audit bench

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project in debug mode"
	@echo "  check      - Check compilation without building"
	@echo "  test       - Run all tests"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Run Clippy linter"
	@echo "  clean      - Clean build artifacts"
	@echo "  run        - Run the application"
	@echo "  release    - Build in release mode"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run  - Run Docker container"
	@echo "  docs       - Generate documentation"
	@echo "  audit      - Security audit"
	@echo "  bench      - Run benchmarks"

# Build targets
build:
	cargo build

check:
	cargo check

test:
	cargo test

test-watch:
	cargo watch -x test

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy -- -D warnings

clippy-fix:
	cargo clippy --fix -- -D warnings

clean:
	cargo clean

run:
	cargo run

release:
	cargo build --release

# Docker targets
docker-build:
	docker build -t rag-system:latest .

docker-run:
	docker run -it --rm \
		-v $(PWD)/documents:/app/documents \
		-e OPENAI_API_KEY=$(OPENAI_API_KEY) \
		rag-system:latest

docker-dev:
	docker run -it --rm \
		-v $(PWD):/app \
		-v cargo-cache:/root/.cargo \
		-v target-cache:/app/target \
		-w /app \
		rust:1.75-slim \
		cargo run

# Documentation and analysis
docs:
	cargo doc --no-deps --open

docs-build:
	cargo doc --no-deps

audit:
	cargo audit

audit-fix:
	cargo audit fix

bench:
	cargo bench

# Development helpers
deps:
	cargo install cargo-watch cargo-audit cargo-outdated

update-deps:
	cargo update

outdated:
	cargo outdated

# CI/CD helpers
ci-local: check fmt-check clippy test

ci-full: check fmt-check clippy test audit

# Install development tools
install-tools:
	cargo install cargo-watch cargo-audit cargo-outdated cargo-tree cargo-bloat

# Show dependency tree
deps-tree:
	cargo tree

# Check binary size
bloat:
	cargo bloat --release

# Run with environment variables
run-dev:
	OPENAI_API_KEY=$(OPENAI_API_KEY) \
	RUST_LOG=debug \
	cargo run

# Example usage
example:
	@echo "Example commands:"
	@echo "  make build"
	@echo "  make test"
	@echo "  make clippy"
	@echo "  make run"
	@echo "  OPENAI_API_KEY=your-key make run-dev"