#![warn(missing_docs)]
pub mod clients;
pub mod error;
mod transports;
pub mod models;

#[cfg(not(target_arch = "wasm32"))]
pub mod blog {
    tonic::include_proto!("blog");
}


