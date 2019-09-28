#
# A stage for fetching data from Git and URL.
#
FROM ubuntu:18.04 AS ubuntu-fetch

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
    ca-certificates \
    curl \
    git-core \
  && rm --recursive --force /var/lib/apt/lists/*

#
# A stage fetching llvm-xtensa repository.
#
FROM ubuntu-fetch AS repo-llvm-xtensa

RUN git clone https://github.com/espressif/llvm-xtensa.git /root/repo/llvm-xtensa \
  && cd /root/repo/llvm-xtensa/ \
  && git checkout 757e18f722dbdcd98b8479e25041b1eee1128ce9

#
# A stage fetching clang-xtensa repository.
#
FROM ubuntu-fetch AS repo-clang-xtensa

RUN git clone https://github.com/espressif/clang-xtensa.git /root/repo/clang-xtensa \
  && cd /root/repo/clang-xtensa/ \
  && git checkout 248d9ce8765248d953c3e5ef4022fb350bbe6c51

#
# A stage building llvm-xtensa.
#
FROM ubuntu:18.04 AS build-llvm-xtensa

COPY --from=repo-llvm-xtensa /root/repo/llvm-xtensa /root/repo/llvm-xtensa
COPY --from=repo-clang-xtensa /root/repo/clang-xtensa /root/repo/llvm-xtensa/tools/clang
RUN ls -l /root/repo/llvm-xtensa && ls -l /root/repo/llvm-xtensa/tools/clang
