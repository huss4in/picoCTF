# Base image
FROM alpine

# Working Directory
WORKDIR /picoctf

# Get files
RUN wget https://mercury.picoctf.net/static/704f877da185904ec3992e7255a15c6c/flag

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
