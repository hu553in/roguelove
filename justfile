build:
    cargo build

test:
    cargo test

run:
    cargo run

fmt:
    cargo fmt

fmt-check:
    cargo fmt --all -- --check

clippy-check:
    cargo clippy -- -D warnings

clippy-fix:
    cargo clippy --fix -- -D warnings

clippy-fix-dirty:
    cargo clippy --fix --allow-dirty -- -D warnings

all-checks: build test fmt-check clippy-check
