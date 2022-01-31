#!/bin/bash

set -e

#-----------------------------------------------------------#
# Go to the script's directory
pushd "$(dirname "$(readlink -f "$BASH_SOURCE")")" >/dev/null

source ./color.sh

# if $1 is empty
if [ -z "$1" ]; then
    echo "Usage: $0 <PATH>"
    exit 1
fi

# If there are multiple arguments, use quiet build
if [ "$#" -gt 3 ]; then
    QUIET='--quiet'
fi

for path in "$@"; do

    file=$(echo "$path"* | sed -e "s/^\w*\///")
    tag="${file%%.*}"

    printf "\n%s %s <-- %s...\n" "$(color YELLOW "Building" | color BOLD)" "$(color LIGHT_BLUE "picoctf:$tag" | color BLINK)" "$(color MAGENTA "$file")"

    DOCKER_SCAN_SUGGEST=false docker build $QUIET --tag "picoctf:$tag" "$path"*
done

set +e

for path in "$@"; do

    file=$(echo "$path"* | sed -e "s/^\w*\///")
    tag="${file%%.*}"

    printf "\n%s %s...\n" "$(color GREEN "Running" | color BOLD)" "$(color BLUE "picoctf:$tag")"

    docker run --rm -ti --name "picoctf-$tag" "picoctf:$tag"
done

# Return to the previous directory
popd >/dev/null
