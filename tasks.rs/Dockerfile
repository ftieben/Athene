FROM rust:1.50.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/api-service
COPY . .
RUN cargo install --path .

# -------------------------------------------------
FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/local/cargo/bin/athene /usr/local/bin/athene
CMD ["athene"]