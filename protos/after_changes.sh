# for auth service
cp protos/posts.proto auth/
cp protos/stats.proto auth/
# for posts service
protoc --go_out=posts/ --go_opt=paths=source_relative \
    --go-grpc_out=posts/ --go-grpc_opt=paths=source_relative \
    protos/posts.proto
# for stats service
python3 -m grpc_tools.protoc -Iprotos --python_out=stats \
    --pyi_out=stats --grpc_python_out=stats \
    protos/stats.proto
