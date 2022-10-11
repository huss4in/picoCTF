# 104

FROM python:alpine

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/77a2b202236aa741e988581e78d277a6/enc

CMD set -x; cat enc | python -c 'print("".join(chr(c) for c in input().encode("utf-16-be")))'
