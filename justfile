styles:
  sass -q --style=compressed --no-source-map styles/main.scss static/styles.css

sitemap:
  URL='https://adamperkowski.dev' bash scripts/sitemap.sh

webhook:
  URL='https://adamperkowski.dev' OWNER='adamperkowski' bash scripts/webhook-all.sh

run:
  @just styles
  @just sitemap
  cargo watch -x run

deploy:
  rm -f static/{styles.css,sitemap.xml}
  @just styles
  @just sitemap
