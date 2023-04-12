# 132

FROM alpine

WORKDIR /picoctf

RUN apk update && apk add --no-cache curl

CMD set -x; curl -sI http://mercury.picoctf.net:45028 | grep flag
