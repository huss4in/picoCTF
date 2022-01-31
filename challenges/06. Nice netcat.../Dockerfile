# Base image
FROM python:3.9-alpine

# Working Directory
WORKDIR /picoctf

# Copy files
COPY solution.py ./

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
