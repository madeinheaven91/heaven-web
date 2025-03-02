FROM rust:1.85-slim

ARG FRONTEND_PORT
ENV FRONTEND_PORT=$FRONTEND_PORT
COPY Trunk.toml .
RUN sed -i -e "s/frontend_port/${FRONTEND_PORT}/g" Trunk.toml
RUN mv Trunk.toml /tmp/Trunk.toml.tmp

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-pack for testing
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install trunk
ADD https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz ./tmp
RUN cd /tmp && tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && chmod +x trunk && mv trunk /bin

# Setup working directory
WORKDIR /usr/src/app
# COPY ./crates/conduit-wasm .

# Caching dependencies
COPY dummy2.rs .
COPY Cargo.lock .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy2.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy2.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo build --release

RUN mv /tmp/Trunk.toml.tmp Trunk.toml
EXPOSE ${FRONTEND_PORT}

CMD [ "trunk", "serve" ]
