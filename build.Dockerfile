FROM alpine

RUN apk add --no-cache ca-certificates

COPY target/x86_64-unknown-linux-musl/release/nzoths-trident /usr/local/bin/nzoths-trident

FROM scratch

COPY --from=0 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=0 /usr/local/bin/nzoths-trident /usr/local/bin/nzoths-trident

ENTRYPOINT ["/usr/local/bin/nzoths-trident"]