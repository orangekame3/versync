# Default recipe
default: check

# Build the project
build:
    cargo build

# Build release binary
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --check

# Run all checks (test + lint + fmt)
check: test lint fmt-check

# Verify version sync
version-check:
    cargo run --quiet -- check

# Apply version to all targets
version-apply:
    cargo run --quiet -- apply

# Create git tag
version-tag:
    cargo run --quiet -- tag

# Install locally
install:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean
