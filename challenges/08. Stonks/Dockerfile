# Base image
FROM alpine AS build

# Get files
WORKDIR /
RUN wget https://mercury.picoctf.net/static/17ba7f9351aca192c45833c658742fe5/vuln.c


# Base image
FROM python:3.9-alpine

# Working Directory
WORKDIR /picoctf

# Install dependencies
RUN apk add --no-cache netcat-openbsd
RUN pip install --no-cache-dir termcolor

# Copy files
COPY --from=build vuln.c ./
COPY . .

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
