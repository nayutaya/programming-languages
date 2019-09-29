set -eu
cd -- `dirname -- $0`
docker container run --interactive --tty --rm \
  --volume $(pwd):/workspace \
  ${USER}/rust-xtensa \
  /bin/bash
