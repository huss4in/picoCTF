# Base image
FROM alpine


# Working Directory
WORKDIR /picoctf

# Download
ENV flag "cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_jdJBFOXJ}"

# Flag style
ARG FLAG_STYLE "%s\n"
ENV FLAG_STYLE $FLAG_STYLE

# Solution
CMD printf $FLAG_STYLE $(set -x; \
  echo $flag | tr 'a-zA-Z' 'n-za-mN-ZA-M'; \
)
