FROM ubuntu:18.04

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

RUN git clone https://github.com/espressif/llvm-xtensa.git /root/llvm-xtensa \
  && cd /root/llvm-xtensa/ \
  && git checkout 757e18f722dbdcd98b8479e25041b1eee1128ce9
RUN git clone https://github.com/espressif/clang-xtensa.git /root/llvm-xtensa/tools/clang \
  && cd /root/llvm-xtensa/tools/clang/ \
  && git checkout 248d9ce8765248d953c3e5ef4022fb350bbe6c51
