#! /usr/bin/env bash

mkdir -p .git/hooks

{ tee .git/hooks/pre-commit << EOF
#!/bin/sh

sed -i "s/^Last update: .*/Last update: \$(date '+%Y\/%m\/%d')/" static/humans.txt
exec git add static/humans.txt
EOF
} > /dev/null

chmod +x .git/hooks/pre-commit
