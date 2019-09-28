FROM ubuntu:18.04

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    bison \
    flex \
    gcc \
    git-core \
    gperf \
    libncurses-dev \
    make \
    python \
    python-cryptography \
    python-future \
    python-pip \
    python-pyparsing \
    python-serial \
    python-setuptools \
    wget \
  && rm --recursive --force /var/lib/apt/lists/*

RUN cd /root/ \
  && wget --quiet https://dl.espressif.com/dl/xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz \
  && tar zxfv xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz -C /opt \
  && rm xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz
