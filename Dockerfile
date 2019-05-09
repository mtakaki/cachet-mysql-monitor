FROM rust:1.34 as build

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y musl-tools libssl-dev openssl libssl-dev pkg-config
RUN rustup target add x86_64-unknown-linux-musl
ENV OPENSSL_INSTALLDIR=/usr/local/ssl

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/cachet-mysql-monitor /build-out/

RUN ls /build-out/

FROM scratch

WORKDIR /usr/src/myapp
COPY --from=build /build-out/cachet-mysql-monitor /usr/src/myapp/
COPY --from=build /usr/src/myapp/config.yml /usr/src/myapp/config/
VOLUME /usr/src/myapp/config/

CMD ["/usr/src/myapp/cachet-mysql-monitor", "/usr/src/myapp/config/config.yml"]
