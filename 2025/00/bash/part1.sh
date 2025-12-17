#!/usr/bin/env bash

INPUT_FILE=${1:-../input.txt}

while read -r line; do
	echo "$line"
done <"$INPUT_FILE"
