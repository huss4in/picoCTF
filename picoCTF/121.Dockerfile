# 121

FROM debian:bullseye-slim

RUN apt update && apt install -y wget

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/9cc50abd5b012891d5a1132e05f15a07/keygenme-trial.py

CMD set -x; get(){ grep -m1 -oP "$1" keygenme-trial.py;}; printf "%s%s%s" \
    "$(get '(?<=key_part_static1_trial = ").*(?=")')" \
    "$(get '(?<=username_trial = ").*(?=")'| tr -d '\n' | sha256sum | sed -E "s/$(printf "%9s" | sed 's/ /(\\w)/g').*/$(grep -oP '(?<=\.hexdigest\(\)\[)\d(?=\])' keygenme-trial.py | xargs -I N expr 1 + N | xargs printf '\\%d')/g")" \
    "$(get '(?<=key_part_static2_trial = ").*(?=")')" \
    | sed 'a\'

