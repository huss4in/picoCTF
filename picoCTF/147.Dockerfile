# 147

FROM alpine

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/704f877da185904ec3992e7255a15c6c/flag

CMD set -x; cat flag
