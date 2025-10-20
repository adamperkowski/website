run:
  @just build-styles
  cargo shuttle run

watch:
  cargo watch -s 'just build-styles; cargo shuttle run --external'

check:
  cargo fmt --all --check
  cargo clippy -- -D warnings

test:
  cargo test

format:
  cargo fmt --all

build-styles:
  sass --no-source-map --style=compressed -q styles/main.scss static/styles.css

deploy:
  @just clean
  @just build-styles
  @just test
  cargo shuttle deploy

clean:
  rm -f static/main.css
