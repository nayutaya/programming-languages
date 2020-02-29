# Docker上におけるHello World

## チートシート

```sh
cd ~/repo/github.com/nayutaya/programming-languages/rust/docker_hello_world/
export USER_ID=$(id -u)
export GROUP_ID=$(id -g)
docker-compose run shell
docker-compose build hello_world
docker-compose run hello_world
```
