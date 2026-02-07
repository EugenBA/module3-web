//! Библиотека для реализации API  Blog клиента
//!
//! Предоставляет функциональность для взаимодействия с бэкэндом

#![warn(missing_docs)]

pub mod clients;
pub mod error;
mod transports;
pub mod models;

#[allow(missing_docs)]
#[cfg(not(target_arch = "wasm32"))]
pub mod blog {
    tonic::include_proto!("blog");
}


