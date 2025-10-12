#! /usr/bin/env bash

display_warning() {
  echo "WARNING: $1"
  echo '         press enter to proceed'
  read -r _
}

load_secrets() {
  if [ -f 'Secrets.toml' ]; then
    export $(cat Secrets.toml | sed "s/ = /=/" | tr -d "'")
  else
    display_warning 'Secrets.toml not found'
  fi
}
