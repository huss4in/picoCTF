# Base image
FROM python:alpine

# Working Directory
WORKDIR /picoctf

# Install dependencies
RUN apk add --no-cache curl

# Copy files
COPY . .

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
