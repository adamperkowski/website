styles:
  sass -q --style=compressed --no-source-map styles/main.scss static/styles.css

run:
  @just styles
  cargo shuttle run

deploy:
  @just styles
  cargo shuttle deploy
