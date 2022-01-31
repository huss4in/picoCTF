#!/bin/bash

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

color() {
    printf "${colors[$1]}${2:-$(</dev/stdin)}${colors[END]}"
}
