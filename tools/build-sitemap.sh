#!/usr/bin/env bash
# Generates sitemap.xml and robots.txt into the trunk staging dir from
# the posts/ directory. Invoked as a Trunk post_build hook.
set -euo pipefail

BASE_URL="https://alegemaate.com"
OUT_DIR="${TRUNK_STAGING_DIR:-dist}"
ROOT_DIR="${TRUNK_SOURCE_DIR:-$(pwd)}"
POSTS_DIR="${ROOT_DIR}/posts"

today=$(date -u +%Y-%m-%d)

{
  echo '<?xml version="1.0" encoding="UTF-8"?>'
  echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'

  for path in "/" "/projects" "/blog"; do
    echo "  <url><loc>${BASE_URL}${path}</loc><lastmod>${today}</lastmod></url>"
  done

  if [ -d "${POSTS_DIR}" ]; then
    for f in "${POSTS_DIR}"/*.md; do
      [ -e "$f" ] || continue
      slug=$(basename "$f" .md)
      date=$(awk '/^---/{c++; next} c==1 && /^date:/ {sub(/^date:[[:space:]]*/, ""); print; exit}' "$f")
      date="${date:-$today}"
      echo "  <url><loc>${BASE_URL}/blog/${slug}</loc><lastmod>${date}</lastmod></url>"
    done
  fi

  echo '</urlset>'
} > "${OUT_DIR}/sitemap.xml"

cat > "${OUT_DIR}/robots.txt" <<EOF
User-agent: *
Allow: /

Sitemap: ${BASE_URL}/sitemap.xml
EOF
