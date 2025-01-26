styles:
  sass -q --style=compressed --no-source-map styles/main.scss static/styles.css

run:
  @just styles
  shuttle run

deploy:
  rm -f static/styles.css
  @just styles
  shuttle deploy
