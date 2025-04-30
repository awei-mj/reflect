#!/bin/zsh
hexdump -v -e '/1 "%u\n"' $1 | jq -cs . | xclip

