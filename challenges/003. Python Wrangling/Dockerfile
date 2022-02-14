# Build image
FROM alpine AS build

# Get files
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/ende.py
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/pw.txt
RUN wget https://mercury.picoctf.net/static/5c4c0cbfbc149f3b0fc55c26f36ee707/flag.txt.en


# Base image
FROM python:3.9-alpine

# Working Directory
WORKDIR /picoctf

# Copy files
COPY --from=build ende.py pw.txt flag.txt.en ./

# Install requirements
RUN apk add --no-cache gcc musl-dev libffi-dev
RUN pip install --no-cache-dir cryptography

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.sh .
CMD [ "/picoctf/solution.sh" ]
