#![warn(missing_docs)]
mod error;
mod grpc_client;
mod http_client;

mod clients;
mod models;

pub mod blog {
    tonic::include_proto!("blog");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
