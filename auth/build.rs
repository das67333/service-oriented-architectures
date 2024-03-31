fn main() {
    tonic_build::compile_protos("posts.proto").unwrap()
}
