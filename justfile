run:
  cargo shuttle run

watch:
  cargo watch -s 'cargo shuttle run --external'

check:
  cargo fmt --all --check
  cargo clippy -- -D warnings

test:
  cargo test

format:
  cargo fmt --all

deploy:
  @just test
  cargo shuttle deploy
