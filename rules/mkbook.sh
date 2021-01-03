#!/bin/bash

# `cd` into the directory where this script lives
cd "$(dirname "${BASH_SOURCE[0]}")"

for f in *.dl; do
    if [[ $f == all.dl ]]; then
        continue
    fi
    echo $f

    ../literate.py "$f" > "../book/src/rules/${f%.*}.md"

done
