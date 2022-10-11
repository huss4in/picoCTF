# 162

FROM python:3.8-slim-buster

RUN apt update && apt install -y git wget gcc libgmp3-dev libmpc-dev

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/b9ddda080c56fb421bf30409bec3460d/values

RUN git clone https://github.com/Ganapati/RsaCtfTool.git && pip install -r RsaCtfTool/requirements.txt

CMD set -x; get(){ grep -m1 -oP "(?<=$1: )\d+" values;}; python RsaCtfTool/RsaCtfTool.py -n $(get n) -e $(get e) --uncipher $(get c) | grep utf-8
