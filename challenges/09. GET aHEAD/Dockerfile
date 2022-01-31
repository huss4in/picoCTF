# Base image
FROM alpine

# Working Directory
WORKDIR /picoctf

# Install dependencies
RUN apk add --no-cache curl

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.sh .
CMD [ "/picoctf/solution.sh" ]
