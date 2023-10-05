# shellcheck disable=SC2231
for file in bundled_icons/svg/*.svg; do inkscape "$file" -o "${file%svg}png"; done