FROM rust:1.41-slim-buster AS development
WORKDIR /root/
RUN rustup component add rustfmt
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
  && echo "// dummy" > src/lib.rs \
  && cargo build --release \
  && rm -rf src/
COPY src ./src
COPY proto ./proto
RUN cargo install --path . --root /opt

FROM debian:buster-slim
COPY --from=development /opt /opt
CMD ["/opt/bin/server"]
