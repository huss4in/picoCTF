# 186

FROM ubuntu

RUN apt-get update && apt-get install -y wget

WORKDIR /picoctf

RUN wget https://mercury.picoctf.net/static/d1375e383810d8d957c04eef9e345732/cat.jpg

CMD set -x; grep -aoP "(?<=<cc:license rdf:resource=')\w+(?='\/>)" cat.jpg | base64 -d | sed 'a\'
