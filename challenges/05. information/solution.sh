#!/bin/sh

printf $FLAG_STYLE "$(
    set -x

    # Solution:
    grep -aoP "(?<=<cc:license rdf:resource=')\w+(?='\/>)" cat.jpg | base64 -d
)"
