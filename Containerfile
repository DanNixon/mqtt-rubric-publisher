FROM docker.io/library/rust:alpine3.15 as builder

RUN apk add \
  cmake \
  g++ \
  libc-dev \
  make \
  openssl-dev

COPY . .
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo install \
  --path . \
  --root /usr/local

FROM docker.io/library/alpine:3.15

RUN apk add \
  libgcc \
  tini

COPY --from=builder \
  /usr/local/bin/mqtt-rubric-publisher \
  /usr/local/bin/mqtt-rubric-publisher

RUN mkdir /config

ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/mqtt-rubric-publisher", "--mapping-file", "/config/mapping.toml"]
