# Build image
FROM alpine AS build

# Get files
RUN wget https://mercury.picoctf.net/static/77a2b202236aa741e988581e78d277a6/enc


# Base image
FROM python:alpine

# Working Directory
WORKDIR /picoctf

# Copy files
COPY --from=build enc .

# # Install requirements
RUN pip install --no-cache-dir termcolor

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.py .
CMD [ "/picoctf/solution.py" ]
