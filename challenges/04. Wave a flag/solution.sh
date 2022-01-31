#!/bin/sh

printf $FLAG_STYLE "$(
    set -x

    # Solution:
    ./warm -h | grep -oE 'picoCTF\{\w+\}'
)"
