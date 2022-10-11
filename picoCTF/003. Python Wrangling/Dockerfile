# 166

FROM python:alpine

RUN apk add --no-cache gcc musl-dev libffi-dev
RUN pip install --no-cache-dir cryptography

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/ende.py
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/pw.txt
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/flag.txt.en

CMD set -x; cat pw.txt | python ende.py -d ./flag.txt.en
