#! /usr/bin/env bash

if [ -z "$URL" ]; then
  echo "no URL provided" > /dev/stderr
  exit 1
fi

date='+%Y-%m-%dT%H:%M:%S+00:00'

top=$(cat << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!-- generated automatically -->
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
EOF
)
main=''
bottom='</urlset>'

cd ./templates/pages

for page in *; do
  if [[ "$page" = 'home.tera' ]]; then
    pagename=''
    prio='1.00'
  elif [[ "$page" = 'error.tera' ]]; then
    continue
  else
    pagename="/${page%.tera}"
    prio='0.80'
  fi

  main="$main$(cat << EOF

  <url>
    <loc>$URL$pagename/</loc>
    <lastmod>$(date -u -r "$page" "$date")</lastmod>
    <changefreq>monthly</changefreq>
    <priority>$prio</priority>
  </url>
EOF
)"
done

cd - > /dev/null

{ tee ./static/sitemap.xml << EOF
$top
$main

$bottom
EOF
} > /dev/null
