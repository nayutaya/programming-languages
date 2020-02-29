FROM rust:1.41-buster AS development
WORKDIR /root/
COPY . .
RUN cargo install --path . --root /opt

FROM debian:buster
COPY --from=development /opt /opt
CMD ["/opt/bin/hello_world"]
