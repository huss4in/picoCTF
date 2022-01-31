# Build image
FROM alpine AS build

# Get files
RUN wget https://mercury.picoctf.net/static/77a2b202236aa741e988581e78d277a6/enc

# Base image
FROM python:alpine

# Working Directory
WORKDIR /picoctf

# Copy files
COPY --from=build enc ./
COPY solution.py ./

# # Install requirements
# RUN pip install --no-cache-dir 

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
