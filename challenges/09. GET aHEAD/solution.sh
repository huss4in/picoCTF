#!/bin/sh

printf $FLAG_STYLE "$(
    set -x

    # Solution:
    curl -sI http://mercury.picoctf.net:45028 | grep -oE "picoCTF\{\w+\}"
)"
