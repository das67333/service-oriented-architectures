fn main() {
    tonic_build::compile_protos("posts.proto").unwrap();
    tonic_build::compile_protos("stats.proto").unwrap();
}
