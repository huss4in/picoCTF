# 163

FROM ubuntu

RUN apt-get update && apt-get install -y wget binutils

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/66932732825076cad4ba43e463dae82f/static
RUN wget https://mercury.picoctf.net/static/66932732825076cad4ba43e463dae82f/ltdis.sh

RUN chmod +x ltdis.sh

CMD set -x; grep picoCTF "$(./ltdis.sh static | grep -oP '(?<= )[\w\.]+strings.txt')"
