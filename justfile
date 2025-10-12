run:
  @just styles
  @just sitemap
  cargo shuttle run --external

styles:
  sass -q --style=compressed --no-source-map styles/main.scss static/styles.css

sitemap:
  URL='https://adamperkowski.dev' bash scripts/sitemap.sh

watch:
  cargo watch -s 'just styles; just sitemap; cargo shuttle run --external'

webhook:
  URL='https://adamperkowski.dev' OWNER='adamperkowski' bash scripts/webhook-all.sh

clean:
  rm -f static/{styles.css,sitemap.xml}

deploy:
  @just styles
  @just sitemap
  cargo shuttle deploy
