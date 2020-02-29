# Rust + gRPC

## チートシート

```sh
cd ~/repo/github.com/nayutaya/programming-languages/rust/grpc/
export USER_ID=$(id -u)
export GROUP_ID=$(id -g)
docker-compose run --service-ports shell
docker-compose build grpc_server
docker-compose run grpc_server
docker-compose build grpc_client
docker-compose run grpc_client

cd ~/repo/github.com/nayutaya/programming-languages/rust/grpc/grpc_server/
grpcurl -plaintext \
  -import-path ./proto \
  -proto helloworld.proto \
  -d '{"name": "Tonic"}' \
  localhost:50051 \
  helloworld.Greeter/SayHello
```

## 参考

* [Rust でも gRPC が使いたい！ - Qiita](https://qiita.com/watawuwu/items/114e2674736e44d4b16d)
