#!/bin/bash
set -eu
cd -- `dirname -- $0`
DOCKER_BUILDKIT=1 docker image build \
  --target devel \
  --tag ${USER}/rust-xtensa \
  rust-xtensa
