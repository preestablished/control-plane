#![forbid(unsafe_code)]
//! Shared Project Determinism contract facade.
//!
//! M0 keeps code generation intentionally thin so every repo can compile against one
//! versioned crate while the service implementations are still skeletons.

pub const PROTO_VERSION: &str = "proto-v0.1.0";

#[cfg(feature = "common")]
pub mod common {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct Timestamp {
            pub unix_millis: i64,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct Blake3 {
            pub hash: Vec<u8>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct PageRef {
            pub snapshot_ref: Vec<u8>,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ListPage {
            pub page_size: u32,
            pub page_token: String,
        }

        impl Default for ListPage {
            fn default() -> Self {
                Self {
                    page_size: 50,
                    page_token: String::new(),
                }
            }
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct ListPageOut {
            pub next_page_token: String,
        }
    }
}

#[cfg(feature = "hypervisor")]
pub mod hypervisor {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct SnapshotRef {
            pub hash: Vec<u8>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct Lease {
            pub slot_id: u64,
            pub token: Vec<u8>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct KvmCaps {
            pub user_space_msr: bool,
            pub msr_filter: bool,
            pub dirty_ring: bool,
            pub immediate_exit: bool,
            pub no_in_kernel_irqchip: bool,
        }
    }
}

#[cfg(feature = "snapstore")]
pub mod snapstore {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct NodeMeta {
            pub experiment_id: String,
            pub node_id: u64,
            pub parent_id: Option<u64>,
            pub snapshot_ref: Vec<u8>,
            pub input_log_id: Vec<u8>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct PutSnapshotRequest {
            pub manifest: Vec<u8>,
        }
    }
}

#[cfg(feature = "controlplane")]
pub mod controlplane {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct Budgets {
            pub max_expansions: u64,
            pub max_wall_clock_secs: u64,
            pub max_guest_seconds: u64,
            pub max_snapshot_bytes: u64,
            pub max_tree_depth: u32,
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct BurstParams {
            pub k_bursts_per_expansion: u32,
            pub burst_frames: u32,
            pub guest_seconds_per_job: f64,
            pub synthesizer_profile: String,
        }

        impl Default for BurstParams {
            fn default() -> Self {
                Self {
                    k_bursts_per_expansion: 16,
                    burst_frames: 120,
                    guest_seconds_per_job: 2.0,
                    synthesizer_profile: "weighted_random".to_string(),
                }
            }
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct ExperimentSpec {
            pub budgets: Budgets,
            pub burst: BurstParams,
            pub seed: u64,
        }
    }
}

#[cfg(feature = "orchestrator")]
pub mod orchestrator {
    pub mod v1 {
        use crate::controlplane::v1::ExperimentSpec;

        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct StartExperimentRequest {
            pub experiment_id: String,
            pub spec: ExperimentSpec,
        }
    }
}

#[cfg(feature = "inputsynth")]
pub mod inputsynth {
    pub mod v1 {
        pub const BURST_FORMAT_VERSION: u32 = 1;

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct PadSegment {
            pub start_frame: u32,
            pub frames: u32,
            pub buttons: u32,
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct Burst {
            pub format_version: u32,
            pub pad_segments: Vec<PadSegment>,
        }
    }
}

#[cfg(feature = "policy")]
pub mod policy {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct Token {
            pub token_id: u32,
            pub logprob: f32,
        }
    }
}

#[cfg(feature = "scorer")]
pub mod scorer {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct ScoreResult {
            pub node_id: u64,
            pub progress_score: f64,
            pub novelty_score: f64,
            pub duplicate: bool,
        }
    }
}

#[cfg(feature = "replay")]
pub mod replay {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct SubmitReplayJobRequest {
            pub experiment_id: String,
            pub target_node_id: u64,
            pub verify_only: bool,
        }
    }

    pub mod agent {
        pub mod v1 {
            #[derive(Clone, Debug, Default, PartialEq, Eq)]
            pub struct PingResponse {
                pub version: String,
                pub hypervisor_reachable: bool,
            }
        }
    }
}

#[cfg(feature = "observatory")]
pub mod observatory {
    pub mod v1 {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct EventEnvelope {
            pub run_id: String,
            pub seq: u64,
            pub source_service: String,
            pub event_type: String,
            pub payload_json: String,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_proto_tag() {
        assert_eq!(crate::PROTO_VERSION, "proto-v0.1.0");
    }

    #[cfg(feature = "inputsynth")]
    #[test]
    fn burst_version_is_stable() {
        assert_eq!(crate::inputsynth::v1::BURST_FORMAT_VERSION, 1);
    }
}
