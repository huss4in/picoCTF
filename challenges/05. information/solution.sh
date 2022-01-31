#!/bin/sh

set -x

echo $(grep -aoP "(?<=<cc:license rdf:resource=')\w+(?='\/>)" cat.jpg | base64 -d)
