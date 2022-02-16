#!/bin/bash

set -e

declare -A colors

colors=(
    ['END']="\e[0m"
    ['BOLD']="\e[1m"
    ['DEM']="\e[2m"
    ['ITALIC']="\e[3m"
    ['UNDERLINED']="\e[4m"
    ['BLINK']="\e[5m"
    ['INVERTED']="\e[7m"
    ['HIDDEN']="\e[8m"
    ['CROSSED']="\e[9m"
    ['BLACK']="\e[30m"
    ['RED']="\e[31m"
    ['GREEN']="\e[32m"
    ['YELLOW']="\e[33m"
    ['BLUE']="\e[34m"
    ['MAGENTA']="\e[35m"
    ['CYAN']="\e[36m"
    ['GRAY']="\e[37m"
    ['DEF']="\e[39m"
    ['DARK_GREY']="\e[90m"
    ['LIGHT_RED']="\e[91m"
    ['LIGHT_GRAY']="\e[92m"
    ['LIGHT_YELLOW']="\e[93m"
    ['LIGHT_BLUE']="\e[94m"
    ['LIGHT_MAGENTA']="\e[95m"
    ['LIGHT_CYAN']="\e[96m"
    ['WHITE']="\e[97m"
)

FLAG_STYLE="\e[1;4;5;31m%s\e[0m\n"

#-----------------------------------------------------------#
# Go to the script's directory
pushd "$(dirname "$(readlink -f "$BASH_SOURCE")")" >/dev/null

color() {
    printf "${colors[$1]}${2:-$(</dev/stdin)}${colors[END]}"
}

# if $1 is empty
if [ -z "$1" ]; then
    echo "Usage: $0 <PATH>"
    exit 1
fi

# # If there are multiple arguments, use quiet build
# if [ "$#" -gt 1 ]; then
#     QUIET='--quiet'
# fi

for path in "$@"; do

    file=$(echo "$path"* | sed -e "s/^\w*\///")
    tag="${file%%.*}"

    printf "\n%s %s <- %s..." "$(color YELLOW "🔨 Building")" "$(color LIGHT_BLUE "picoctf:$tag")" "$(color MAGENTA "$file")"
    DOCKER_SCAN_SUGGEST=false docker build --quiet --tag "picoctf:$tag" --build-arg FLAG_STYLE="$FLAG_STYLE" "$path"* >/dev/null

    printf "\r%s %s\n" "$(color GREEN " 🚀 Running")" "$(color LIGHT_BLUE "picoctf:$tag")"
    docker run --rm -ti --name "picoctf-$tag" "picoctf:$tag"
    echo
done

# Return to the previous directory
popd >/dev/null
