# 162

FROM python:alpine

RUN apk update && apk add --no-cache git wget

WORKDIR /picoctf

RUN pip install --no-cache-dir factordb-pycli gmpy2

RUN wget https://mercury.picoctf.net/static/b9ddda080c56fb421bf30409bec3460d/values

COPY ./rsadecrypt.py .

CMD set -x; python rsadecrypt.py values
