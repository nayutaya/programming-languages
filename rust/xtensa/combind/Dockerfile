#
# A stage for preparing to fetch data from Git and URL.
#
FROM ubuntu:18.04 AS ubuntu-for-fetch

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    ca-certificates \
    curl \
    git-core \
  && rm --recursive --force /var/lib/apt/lists/*

#
# A stage fetching llvm-xtensa.
#
FROM ubuntu-for-fetch AS fetch-llvm-xtensa

RUN git clone https://github.com/espressif/llvm-xtensa.git /root/repo/llvm-xtensa \
  && cd /root/repo/llvm-xtensa/ \
  && git checkout 757e18f722dbdcd98b8479e25041b1eee1128ce9

#
# A stage fetching clang-xtensa.
#
FROM ubuntu-for-fetch AS fetch-clang-xtensa

RUN git clone https://github.com/espressif/clang-xtensa.git /root/repo/clang-xtensa \
  && cd /root/repo/clang-xtensa/ \
  && git checkout 248d9ce8765248d953c3e5ef4022fb350bbe6c51

#
# A stage fetching rust-xtensa.
#
FROM ubuntu-for-fetch AS fetch-rust-xtensa

RUN git clone https://github.com/MabezDev/rust-xtensa.git /root/repo/rust-xtensa \
  && cd /root/repo/rust-xtensa/ \
  && git checkout b365cff41a60df8fd5f1237ef71897edad0375dd

#
# A stage fetching esp-idf.
#
FROM ubuntu-for-fetch AS fetch-esp-idf

RUN ARCHIVE_NAME=xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz \
  && curl -o /root/${ARCHIVE_NAME} https://dl.espressif.com/dl/${ARCHIVE_NAME} \
  && tar zxfv /root/${ARCHIVE_NAME} -C /opt \
  && rm /root/${ARCHIVE_NAME}
RUN git clone --branch v3.3 --depth 1 https://github.com/espressif/esp-idf.git /opt/esp-idf \
  && rm --recursive --force /opt/esp-idf/.git

#
# A stage building llvm-xtensa.
#
FROM ubuntu:18.04 AS build-llvm-xtensa

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    git-core \
    ninja-build \
    python \
  && rm --recursive --force /var/lib/apt/lists/*

COPY --from=fetch-llvm-xtensa /root/repo/llvm-xtensa /root/repo/llvm-xtensa
COPY --from=fetch-clang-xtensa /root/repo/clang-xtensa /root/repo/llvm-xtensa/tools/clang
RUN mkdir --parents /root/build/llvm-xtensa \
  && cd /root/build/llvm-xtensa/ \
  && cmake \
    -D CMAKE_INSTALL_PREFIX="/opt/llvm-xtensa" \
    -D CMAKE_BUILD_TYPE="Release" \
    -D LLVM_TARGETS_TO_BUILD="X86;Xtensa" \
    -G "Ninja" \
    /root/repo/llvm-xtensa \
  && ninja install

COPY --from=fetch-rust-xtensa /root/repo/rust-xtensa /root/repo/rust-xtensa
RUN mkdir --parents /opt/rust-xtensa \
  && cd /root/repo/rust-xtensa/ \
  && ./configure \
    --llvm-root /root/build/llvm-xtensa \
    --prefix /opt/rust-xtensa \
  && ./x.py build \
  && ./x.py install
RUN cp --recursive /root/repo/rust-xtensa/src /opt/rust-xtensa

#
# A stage for development.
#
FROM ubuntu:18.04 AS devel

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    bison \
    ca-certificates \
    curl \
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
  && rm --recursive --force /var/lib/apt/lists/*

COPY --from=build-llvm-xtensa /opt/rust-xtensa /opt/rust-xtensa
COPY --from=fetch-esp-idf /opt/xtensa-esp32-elf /opt/xtensa-esp32-elf
COPY --from=fetch-esp-idf /opt/esp-idf /opt/esp-idf

SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --no-modify-path
RUN source $HOME/.cargo/env \
  && rustup toolchain link xtensa /opt/rust-xtensa \
  && rustup default xtensa \
  && rustup toolchain list
RUN source $HOME/.cargo/env \
  && cargo install cargo-xbuild

ENV PATH /opt/xtensa-esp32-elf/bin:${PATH}
ENV IDF_PATH /opt/esp-idf
ENV XARGO_RUST_SRC /opt/rust-xtensa/src
RUN echo 'source $HOME/.cargo/env' >> ~/.bashrc
