
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Этот код выполняется перед сборкой
   // println!("cargo:rerun-if-changed=proto/blog.proto");
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &["proto/blog.proto"],
            &["proto"],
        )?;
    Ok(())
}
