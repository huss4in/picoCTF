# 156

FROM alpine

WORKDIR /picoctf

CMD set -x; echo '\n' | nc mercury.picoctf.net 7449 | xargs -I {} sh -c 'printf "%b" $(printf "\%03o" {})'
