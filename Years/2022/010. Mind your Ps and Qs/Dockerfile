# 162

FROM python:alpine

RUN apk update && apk add --no-cache git wget

WORKDIR /picoctf

RUN pip install --no-cache-dir factordb-pycli gmpy2

RUN wget https://mercury.picoctf.net/static/b9ddda080c56fb421bf30409bec3460d/values

COPY ./rsadecrypt.py .

CMD set -x; python rsadecrypt.py values


# import re
# import sys
#
# import gmpy2
# from factordb.factordb import FactorDB
#
# if sys.argv.__len__() != 2:
#     print("Usage: python rsadecrypt.py <filename>")
#     exit(1)
#
# filename = sys.argv[1]
#
# c, n, e = (int(x) for x in re.findall(r"(?<=[cne]: )(\d+)", open(filename, "r").read()))
#
# f = FactorDB(n)
# f.connect()
# p, q = f.get_factor_list()
# ph = (p - 1) * (q - 1)
# d = gmpy2.invert(e, ph)
# plaintext = pow(c, d, n)
# print("Flag: {}".format(bytearray.fromhex(format(plaintext, "x")).decode()))
