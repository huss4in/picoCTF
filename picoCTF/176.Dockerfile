# 176

FROM ubuntu

RUN apt-get update && apt-get install -y wget unzip file

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/72712e82413e78cc8aa8d553ffea42b0/Addadshashanammu.zip

CMD set -x; ./$(unzip Addadshashanammu.zip | grep -m1 -oP '(?<=inflating: ).*')
