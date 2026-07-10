.PHONY: help build test clean install dev lint fmt check-all

help:
	@echo "Sustaina Dependency Oracle - Make Targets"
	@echo ""
	@echo "Workspace:"
	@echo "  make install       - Install all dependencies (Node + Rust)"
	@echo "  make build         - Build all components"
	@echo "  make test          - Run all test suites"
	@echo "  make clean         - Clean all build artifacts"
	@echo "  make lint          - Run linters across all components"
	@echo "  make fmt           - Format code across all components"
	@echo "  make check-all     - Run all checks (fmt, lint, test)"
	@echo ""
	@echo "Contract:"
	@echo "  make contract-build    - Build Soroban contract"
	@echo "  make contract-test     - Test Soroban contract"
	@echo ""
	@echo "Oracle Service:"
	@echo "  make oracle-build      - Build Node.js oracle service"
	@echo "  make oracle-dev        - Start oracle in dev mode"
	@echo ""
	@echo "CLI:"
	@echo "  make cli-build         - Build Rust CLI"
	@echo "  make cli-test          - Test Rust CLI"
	@echo ""
	@echo "Frontend:"
	@echo "  make ui-build          - Build Next.js frontend"
	@echo "  make ui-dev            - Start frontend in dev mode"

install:
	cd oracle-service && npm install
	cd apps/explorer-ui && npm install
	rustup update stable

build: build-contract build-oracle build-cli build-ui

test: test-contract test-oracle test-cli

clean:
	cargo clean
	rm -rf oracle-service/dist oracle-service/node_modules
	rm -rf apps/explorer-ui/.next apps/explorer-ui/node_modules
	rm -rf apps/explorer-ui/out

# Contract targets
contract-build:
	cd contracts/identity-registry && cargo build --target wasm32-unknown-unknown --release

contract-test:
	cd contracts/identity-registry && cargo test

build-contract: contract-build

test-contract: contract-test

# Oracle Service targets
oracle-build:
	cd oracle-service && npm run build

oracle-dev:
	cd oracle-service && npm run dev

build-oracle: oracle-build

# CLI targets
cli-build:
	cd cli/sustaina-cli && cargo build --release

cli-test:
	cd cli/sustaina-cli && cargo test

build-cli: cli-build

test-cli: cli-test

# Frontend targets
ui-build:
	cd apps/explorer-ui && npm run build

ui-dev:
	cd apps/explorer-ui && npm run dev

build-ui: ui-build

# Code quality
lint:
	cd contracts/identity-registry && cargo clippy -- -D warnings
	cd cli/sustaina-cli && cargo clippy -- -D warnings
	cd oracle-service && npm run lint || true

fmt:
	cd contracts/identity-registry && cargo fmt
	cd cli/sustaina-cli && cargo fmt
	cd oracle-service && npm run fmt || true

check-all: clean lint test build
