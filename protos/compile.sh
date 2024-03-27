protoc --go_out=posts/ --go_opt=paths=source_relative \
    --go-grpc_out=posts/ --go-grpc_opt=paths=source_relative \
    protos/posts.proto
