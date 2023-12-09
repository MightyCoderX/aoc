#!/usr/bin/env bash

# Finds all days directories
days=$(find . -type d -regextype egrep -regex '.*/[0-9]{2}' -printf '%f\n')
last_day=$(echo "$days" | sort | tail -1)
new_day=$(echo $(( last_day + 1)) | xargs printf '%02d')

echo "Creating day $new_day from template"

mkdir "$new_day"
cp -r ./00/. "$new_day"

echo "Done"

