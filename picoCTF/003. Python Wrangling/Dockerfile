FROM python:3.9-alpine

RUN apk add --no-cache gcc musl-dev libffi-dev
RUN pip install --no-cache-dir cryptography

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/ende.py
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/pw.txt
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/flag.txt.en

ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

CMD printf $FLAG_STYLE $(set -x; \
  cat pw.txt | python ende.py -d ./flag.txt.en | grep -oE "picoCTF\{\w+\}"; \
)
