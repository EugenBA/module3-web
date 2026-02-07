fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("cargo:rerun-if-changed=proto/blog.proto");
        tonic_prost_build::configure()
            .build_server(false)
            .build_client(true)
            .compile_protos(&["proto/blog.proto"], &["proto"])?;
    }
    #[cfg(target_family = "wasm32")]
    {
        println!("cargo:warning=tonic-build not supported on wasm32");
    }
    Ok(())
}
