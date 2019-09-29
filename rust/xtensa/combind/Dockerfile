#
# A stage for fetching data from Git and URL.
#
FROM ubuntu:18.04 AS ubuntu-for-fetch

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    ca-certificates \
    curl \
    git-core \
  && rm --recursive --force /var/lib/apt/lists/*

#
# A stage fetching llvm-xtensa repository.
#
FROM ubuntu-for-fetch AS fetch-llvm-xtensa

RUN git clone https://github.com/espressif/llvm-xtensa.git /root/repo/llvm-xtensa \
  && cd /root/repo/llvm-xtensa/ \
  && git checkout 757e18f722dbdcd98b8479e25041b1eee1128ce9

#
# A stage fetching clang-xtensa repository.
#
FROM ubuntu-for-fetch AS fetch-clang-xtensa

RUN git clone https://github.com/espressif/clang-xtensa.git /root/repo/clang-xtensa \
  && cd /root/repo/clang-xtensa/ \
  && git checkout 248d9ce8765248d953c3e5ef4022fb350bbe6c51

#
# A stage fetching rust-xtensa repository.
#
FROM ubuntu-for-fetch AS fetch-rust-xtensa

RUN git clone https://github.com/MabezDev/rust-xtensa.git /root/repo/rust-xtensa \
  && cd /root/repo/rust-xtensa/ \
  && git checkout b365cff41a60df8fd5f1237ef71897edad0375dd

#
# A stage building llvm-xtensa.
#
FROM ubuntu:18.04 AS build-llvm-xtensa

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
RUN mkdir --parents /root/build/rust-xtensa \
  && /root/repo/rust-xtensa/ \
  && ./configure \
    --llvm-root /root/build/llvm-xtensa \
    --prefix /opt/rust-xtensa \
  && ./x.py build \
  && ./x.py install
RUN cp --recursive /root/repo/rust-xtensa/src /opt/rust-xtensa
