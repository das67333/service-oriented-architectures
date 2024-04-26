# for auth service
cp protos/posts.proto auth/
# for posts service
protoc --go_out=posts/ --go_opt=paths=source_relative \
    --go-grpc_out=posts/ --go-grpc_opt=paths=source_relative \
    protos/posts.proto
