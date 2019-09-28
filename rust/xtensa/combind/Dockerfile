#
# An image for fetching data from Git and URL
#
FROM ubuntu:18.04 AS ubuntu-fetch

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    ca-certificates \
    curl \
    git-core \
  && rm --recursive --force /var/lib/apt/lists/*
