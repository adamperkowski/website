#! /bin/sh
mkdir -p .git/hooks

echo '#!/bin/sh

sed -i "s/^Last update: .*/Last update: $(date "+%Y\/%m\/%d")/" static/humans.txt
exec git add static/humans.txt' > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit
