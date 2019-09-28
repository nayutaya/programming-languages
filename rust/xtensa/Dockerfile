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
RUN mkdir --parents /root/build/llvm-xtensa \
  && cd /root/build/llvm-xtensa/ \
  && cmake \
    -D CMAKE_INSTALL_PREFIX="/opt/llvm-xtensa" \
    -D CMAKE_BUILD_TYPE="Release" \
    -D LLVM_TARGETS_TO_BUILD="X86;Xtensa" \
    -G "Ninja" \
    /root/llvm-xtensa \
  && ninja install
RUN git clone https://github.com/MabezDev/rust-xtensa /root/rust-xtensa \
  && cd /root/rust-xtensa/ \
  && mkdir --parents /opt/rust-xtensa \
  && git checkout b365cff41a60df8fd5f1237ef71897edad0375dd \
  && mkdir --parents /root/build/rust-xtensa \
  && ./configure \
    --llvm-root /root/build/llvm-xtensa \
    --prefix /opt/rust-xtensa \
  && ./x.py build \
  && ./x.py install
