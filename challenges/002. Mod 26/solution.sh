#!/bin/sh

printf $FLAG_STYLE "$(
    set -x

    # Solution:
    cat flag | tr 'a-zA-Z' 'n-za-mN-ZA-M'
)"
