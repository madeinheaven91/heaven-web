FROM rust:1.85-slim

# Upgrade and install mold
RUN apt-get update && \
    apt-get upgrade && \
    apt-get install -y mold

WORKDIR /
ARG FRONTEND_PORT
ENV FRONTEND_PORT=$FRONTEND_PORT
COPY Trunk.toml.docker .
RUN sed -i -e "s/frontend_port/${FRONTEND_PORT}/g" Trunk.toml.docker && \
    mv Trunk.toml.docker /tmp/ && \
# Add wasm target
    rustup target add wasm32-unknown-unknown

# Install wasm-pack for testing
# RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


# Install trunk
ADD https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz ./tmp
WORKDIR /tmp
RUN tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && \
    chmod +x trunk && \
    mv trunk /bin

# Setup working directory
WORKDIR /usr/src/app
# COPY ./crates/conduit-wasm .

# Caching dependencies
COPY dummy.rs .
COPY Cargo.lock .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release && \
    sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN mv /tmp/Trunk.toml.docker Trunk.toml && \
    trunk build --release
EXPOSE ${FRONTEND_PORT}

CMD [ "trunk", "serve", "--no-autoreload", "--release" ]

# Stage 1: Build the Yew app
# FROM rust:1.85-slim AS builder
#
# WORKDIR /
# ARG FRONTEND_PORT
# ENV FRONTEND_PORT=$FRONTEND_PORT
#
# COPY Trunk.toml .
# RUN sed -i -e "s/frontend_port/${FRONTEND_PORT}/g" Trunk.toml && \
#     mv Trunk.toml /tmp/Trunk.toml.tmp && \
#     rustup target add wasm32-unknown-unknown
#
# # Install Trunk
# ADD https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz /tmp
# WORKDIR /tmp
# RUN tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && \
#     chmod +x trunk && \
#     mv trunk /bin
#
# # Setup working directory
# WORKDIR /usr/src/app
#
# # Cache dependencies
# COPY dummy.rs .
# COPY Cargo.lock .
# COPY Cargo.toml .
# RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
# RUN cargo build --release && \
#     sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
#
# # Copy the actual source code
# COPY . .
# RUN mv /tmp/Trunk.toml.tmp Trunk.toml && \
#     trunk build --release
#
# # Stage 2: Serve the built files using a lightweight server
# # FROM gcr.io/distroless/base-debian12 AS runtime
# #
# # WORKDIR /app
# # COPY --from=builder /usr/src/app/dist /app/dist
# # COPY --from=builder /usr/bin/sh /usr/bin
# #
# # EXPOSE ${FRONTEND_PORT}
# #
# # RUN apt-get update && apt-get install -y busybox && rm -rf /var/lib/apt/lists/*
# #
# # # Serve static files using a minimal HTTP server
# # CMD ["busybox", "httpd", "-f", "-p", "0.0.0.0:${FRONTEND_PORT}", "-h", "/app/dist"]
# FROM caddy:2.7.6-alpine AS runtime
#
# WORKDIR /app
# COPY --from=builder /usr/src/app/dist /app/dist
# COPY --from=builder /usr/src/app/dist/index.html ./dist/index.html
#
# EXPOSE ${FRONTEND_PORT}
# CMD ["caddy", "file-server", "--root", "/app/dist", "--listen", "0.0.0.0:${FRONTEND_PORT}"]
