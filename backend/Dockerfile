# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:1.85-slim as build

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev

# Setup working directory
WORKDIR /usr/src/hw-backend

# We'll get to what this file is below!
COPY dummy.rs /usr/src/hw-backend
# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock /usr/src/hw-backend
COPY Cargo.toml /usr/src/hw-backend
# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --release
# Now return the file back to normal
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
# Copy the rest of the files into the container
COPY . /usr/src/hw-backend
# Now this only builds our changes to things like src

# Build application
RUN cargo build --release

# Debugging purposes
# ENTRYPOINT [ "cargo", "run" ]
# ENTRYPOINT ["tail", "-f", "/dev/null"]

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy `libpq` dependencies into the image (Required by diesel)
# - Copy application files into the image
# ---------------------------------------------------

FROM gcr.io/distroless/cc-debian12

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
ARG ARCH=x86_64

# # libpq related (required by diesel)
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
# # COPY --from=build /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
# # COPY --from=build /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=build /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
# # COPY --from=build /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
# # COPY --from=build /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
# FIXME: bloats the image. I don't know which libraries are installed with libpq, so right now it is what it is
COPY --from=build /usr/lib/${ARCH}-linux-gnu/* /usr/lib/${ARCH}-linux-gnu/


# # Application files
COPY --from=build /usr/bin/sh /usr/bin
COPY --from=build /usr/src/hw-backend/target/release/hw-backend /usr/local/bin/hw-backend

EXPOSE ${SERVER_PORT}

ENTRYPOINT ["hw-backend"]

# source: https://www.codefeetime.com/post/docker-config-for-actix-web-diesel-and-postgres/
