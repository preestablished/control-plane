#![forbid(unsafe_code)]

pub const PROTO_VERSION: &str = "proto-v0.0.1";

#[cfg(feature = "scratch")]
pub mod scratch {
    pub mod shared {
        pub mod v1 {
            tonic::include_proto!("determinism.scratch.shared.v1");
        }
    }

    pub mod v1 {
        tonic::include_proto!("determinism.scratch.v1");
    }
}
