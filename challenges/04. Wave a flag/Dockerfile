# Base image
FROM alpine AS build

# Get files
WORKDIR /
RUN wget https://mercury.picoctf.net/static/beec4f433e5ee5bfcd71bba8d5863faf/warm


# Base image
FROM ubuntu

# Working Directory
WORKDIR /picoctf

# Copy files
COPY --from=build warm ./

# Setup
RUN chmod +x warm

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
