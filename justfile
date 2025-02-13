styles:
  sass -q --style=compressed --no-source-map styles/main.scss static/styles.css

sitemap:
  URL='https://adamperkowski.dev' bash scripts/sitemap.sh

run:
  @just styles
  @just sitemap
  # shuttle run
  cargo watch -x 'shuttle run'

deploy:
  rm -f static/{styles.css,sitemap.xml}
  @just styles
  @just sitemap
  shuttle deploy
