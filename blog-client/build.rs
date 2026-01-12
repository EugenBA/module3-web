fn main() {
    // Этот код выполняется перед сборкой

    println!("cargo:rerun-if-changed=proto/blog.proto")
}

