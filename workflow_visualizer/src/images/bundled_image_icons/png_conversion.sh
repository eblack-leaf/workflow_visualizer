# shellcheck disable=SC2231
for file in svg/*.svg; do inkscape "$file" -o "${file%svg}png"; done