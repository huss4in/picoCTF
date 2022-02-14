# # Base image
# FROM alpine AS build

# # Get files
# WORKDIR /
# RUN wget https://mercury.picoctf.net/static/17ba7f9351aca192c45833c658742fe5/vuln.c


# Base image
FROM python:alpine

# Working Directory
WORKDIR /picoctf

# # Copy files
# COPY --from=build vuln.c .

# Install dependencies
RUN apk add --no-cache netcat-openbsd
RUN pip install --no-cache-dir termcolor

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.py .
CMD [ "/picoctf/solution.py" ]
