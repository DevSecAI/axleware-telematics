# AXL-IAC-006: runs as root.
FROM rust:1.78
WORKDIR /src
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["/src/target/release/axleware-telematics"]
