#! /usr/bin/env bash

if [ -z "$URL" ]; then
  echo "no URL provided" > /dev/stderr
  exit 1
fi

. scripts/common.sh

display_warning 'this script WILL DELETE all webhooks containing URL'

repos=$(gh repo list \
  --visibility public \
  --source \
  --no-archived \
  --json name \
  "$OWNER" \
  | jq -r '.[].[]')

for repo in $repos; do
  echo "$OWNER/$repo"

  hooks=$(gh api \
    -H "Accept: application/vnd.github.v3+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "/repos/$OWNER/$repo/hooks")

  hook_ids=$(echo "$hooks" | jq -r --arg url "$URL" '.[] | select(.config.url | contains($url)) | .id')

  for hook_id in $hook_ids; do
    echo "  Deleting hook $hook_id"
    gh api \
      -X DELETE \
      -H "Accept: application/vnd.github.v3+json" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      "/repos/$OWNER/$repo/hooks/$hook_id"
  done
done
