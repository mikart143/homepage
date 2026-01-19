.PHONY: setup install-tools build build-release run test clean docker-build docker-run deny-install deny-check help

help:
	@echo "Available targets:"
	@echo "  make setup          - Install tools and build the project"
	@echo "  make install-tools  - Install cargo utilities (cargo-binstall)"
	@echo "  make build          - Build debug binary"
	@echo "  make build-release  - Build optimized release binary"
	@echo "  make run            - Run the project"
	@echo "  make test           - Run tests"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make docker-build   - Build Docker image"
	@echo "  make docker-run     - Run Docker container"
	@echo "  make audit     - Run cargo-deny checks (licenses, advisories, bans)"
	@echo "  make help           - Show this help message"

setup: install-tools build

install-tools:
	cargo install --locked cargo-binstall
	cargo binstall -y cargo-llvm-cov
	cargo binstall -y --secure cargo-nextest
	cargo binstall -y cargo-deny
	cargo binstall -y trunk

audit:
	@command -v cargo-deny >/dev/null 2>&1 || { echo "cargo-deny not found. Run 'make install-tools' first."; exit 1; }
	cargo deny check

build:
	trunk build

build-release:
	trunk build --release

run:
	trunk serve --open

test:
	cargo llvm-cov nextest --all-features --no-tests warn

clean:
	cargo clean

docker-build:
	docker build -t homepage .

docker-run: docker-build
	docker run -p 8080:8080 homepage

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check
