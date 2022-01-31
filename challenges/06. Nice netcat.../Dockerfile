# Base image
FROM python:alpine

# Working Directory
WORKDIR /picoctf

# # Install requirements
RUN pip install --no-cache-dir termcolor

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.py .
CMD [ "/picoctf/solution.py" ]
