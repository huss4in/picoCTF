# 121

FROM python:slim

RUN apt update && apt install -y wget

RUN pip install cryptography

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/9cc50abd5b012891d5a1132e05f15a07/keygenme-trial.py

CMD set -x; get(){ grep -m1 -oP "$1" keygenme-trial.py;}; printf "%s%s%s" \
    "$(get '(?<=key_part_static1_trial = ").*(?=")')" \
    "$(python -c "import hashlib;x=hashlib.sha256($(get '(?<=bUsername_trial = )b".*"')).hexdigest();print(x[4]+x[5]+x[3]+x[6]+x[2]+x[7]+x[1]+x[8])")" \
    "$(get '(?<=key_part_static2_trial = ").*(?=")')" \
    | sed 'a\'

