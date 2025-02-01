#!/usr/bin/env sh

# Generate the blanks images once
magick -size 320x280 label:BLANK /tmp/blank.jpg
magick -size 320x280 label:BLANK /tmp/blank.png

# Replace LFS-tracked jpg and png files with the blank versions
for file in $(git lfs ls-files -n); do
    ext="${file##*.}"
    if [[ "$ext" == "jpg" ]]; then
        cp /tmp/blank.jpg "$file"
    elif [[ "$ext" == "png" ]]; then
        cp /tmp/blank.png "$file"
    fi
done
