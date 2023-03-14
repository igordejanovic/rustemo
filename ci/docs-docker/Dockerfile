# This image is hosted at docker hub (igordejanovic/rustemo-docs-ci)
# And used for docs/book build
#
# To build:
#   buildah build -t igordejanovic/rustemo-docs-ci .
#   buildah push --creds igordejanovic localhost/igordejanovic/rustemo-docs-ci docker.io/igordejanovic/rustemo-docs-ci:latest
FROM ubuntu

RUN apt-get update \
    && apt-get install -y wget git plantuml graphviz \
    && rm -rf /var/lib/apt/lists/*
