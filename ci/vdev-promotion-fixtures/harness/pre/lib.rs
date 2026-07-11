#![forbid(unsafe_code)]

pub const PROTO_VERSION: &str = "proto-v0.0.0";

#[cfg(feature = "scratch")]
pub mod scratch {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct PromoteRequest {
            pub value: String,
        }
    }
}
