#!/bin/sh

printf $FLAG_STYLE "$(
    set -x

    # Solution:
    cat pw.txt | python ende.py -d ./flag.txt.en | grep -oE "picoCTF\{\w+\}"
)"
