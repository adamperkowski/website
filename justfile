styles:
  mkdir -p build/static
  sass -q --style=compressed --no-source-map styles/main.scss build/static/styles.css

sitemap:
  URL='https://adamperkowski.dev' bash scripts/sitemap.sh

build:
  rm -rf build
  mkdir -p build/projects build/donate build/legal build/static
  @just sitemap
  cargo run
  cp -R static/* build/static
  cp static/robots.txt static/humans.txt static/favicon.ico static/sitemap.xml build
