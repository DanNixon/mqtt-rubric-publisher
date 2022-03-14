# MQTT DAPNET rubric publisher [![CI](https://github.com/DanNixon/mqtt-rubric-publisher/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/DanNixon/mqtt-rubric-publisher/actions/workflows/ci.yml)

Tool to publish news to [DAPNET](https://www.hampager.de/) [rubrics](https://hampager.de/dokuwiki/doku.php#rubrics) via [MQTT](https://mqtt.org/).

## Usage

See `mqtt-rubric-publisher --help`.

A mapping file is used to define how MQTT topics map to DAPNET rubrics.
An example of such file is provided [here](./examples/mapping.toml).

## Deployment

A container image is published.
Use it however you like.

e.g. via Podman:
```sh
podman run \
  --rm -it \
  -e RUST_LOG=debug \
  -e DAPNET_USERNAME="<username>" \
  -e DAPNET_PASSWORD="<password>" \
  -e MQTT_BROKER=broker.hivemq.com \
  -v ./examples:/config \
  ghcr.io/DanNixon/mqtt-rubric-publisher:latest
```
