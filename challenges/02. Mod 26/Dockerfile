# Base image
FROM alpine

# Working Directory
WORKDIR /picoctf

# Get files
RUN echo "cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_jdJBFOXJ}" > flag

# Flag style
ARG FLAG_STYLE="%s\n"
ENV FLAG_STYLE=$FLAG_STYLE

# Run solution
COPY solution.sh ./
CMD [ "/picoctf/solution.sh" ]
