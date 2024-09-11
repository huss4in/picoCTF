# 170

FROM ubuntu

RUN apt-get update && apt-get install --no-install-recommends -y wget

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/beec4f433e5ee5bfcd71bba8d5863faf/warm && chmod +x warm

CMD set -x; ./warm -h
