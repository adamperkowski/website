#! /bin/bash

export $(cat Secrets.toml | sed "s/ = /=/" | tr -d "'")
