#! /bin/bash

. scripts/common.sh

display_warning 'this script may create duplicate webhooks'

load_secrets

repos=$(gh repo list \
  --visibility public \
  --source \
  --no-archived \
  --json name \
  "$OWNER" \
  | jq -r '.[].[]')

for repo in $repos; do
  echo "$OWNER/$repo"
  gh api \
    --method POST \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "/repos/$OWNER/$repo/hooks" \
    -f 'name=web' -F 'active=true' -F 'events[]=star' -f "config[url]=$URL/api/github" -f 'config[content_type]=json' -f 'config[insecure_ssl]=0' -f "config[secret]=$GITHUB_SECRET"
done
