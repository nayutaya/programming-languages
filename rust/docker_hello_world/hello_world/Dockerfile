FROM rust:1.41-slim-buster AS development
WORKDIR /root/
COPY . .
RUN cargo install --path . --root /opt

FROM debian:buster-slim
COPY --from=development /opt /opt
CMD ["/opt/bin/hello_world"]
