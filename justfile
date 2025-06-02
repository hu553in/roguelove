build:
    cargo build --verbose

test:
    cargo test --verbose

run:
    cargo run --verbose

lint:
    cargo clippy --verbose -- -D warnings

lint-fix:
    cargo clippy --verbose --fix -- -D warnings

lint-fix-dirty:
    cargo clippy --verbose --fix --allow-dirty -- -D warnings
