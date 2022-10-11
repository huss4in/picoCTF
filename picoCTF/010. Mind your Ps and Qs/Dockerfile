# 162

FROM python:3.8-slim-buster

RUN apt update && apt install -y git wget gcc libgmp3-dev libmpc-dev

WORKDIR /picoctf

RUN git clone https://github.com/Ganapati/RsaCtfTool.git && pip install -r RsaCtfTool/requirements.txt

WORKDIR /picoctf/RsaCtfTool

RUN wget https://mercury.picoctf.net/static/b9ddda080c56fb421bf30409bec3460d/values

CMD set -x; get(){ grep -oP "(?<=$1: )\d+" values;}; python RsaCtfTool.py --uncipher $(get c) -n $(get n) -e $(get e) | tac | grep -m1 -a utf-8
