#![forbid(unsafe_code)]
//! Shared Project Determinism contract facade.
//!
//! Shared handwritten M0 facades plus generated Phase 4 service contracts.

pub const PROTO_VERSION: &str = "proto-v0.2.0";

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
        tonic::include_proto!("determinism.inputsynth.v1");
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
        tonic::include_proto!("determinism.scorer.v1");
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
        assert_eq!(crate::PROTO_VERSION, "proto-v0.2.0");
    }

    #[cfg(feature = "inputsynth")]
    #[test]
    fn burst_version_is_stable() {
        assert_eq!(crate::inputsynth::v1::BURST_FORMAT_VERSION, 1);
    }

    #[cfg(feature = "inputsynth")]
    #[test]
    fn inputsynth_facade_exposes_required_generated_symbols() {
        use crate::inputsynth::v1::{
            health_response, input_synthesizer_client::InputSynthesizerClient,
            load_macro_pack_request, DocumentKind, HealthRequest, HealthResponse,
            LoadMacroPackRequest, MineMacrosRequest, MineMacrosResponse, ModelKind, NodeContext,
            ProposeBurstsRequest, ProvenancedBurst, ScoredBurst,
        };

        let _client_type = std::mem::size_of::<InputSynthesizerClient<tonic::transport::Channel>>();
        let load = LoadMacroPackRequest {
            source: Some(load_macro_pack_request::Source::DocumentYaml(
                b"version: 1\n".to_vec(),
            )),
            kind: DocumentKind::ExperimentConfig as i32,
        };
        let propose = ProposeBurstsRequest {
            experiment_id: "exp-a".to_owned(),
            node_context: Some(NodeContext {
                node_id: "7".to_owned(),
                snapshot_ref: "00".repeat(32),
                depth: 1,
                node_score: 1.25,
                novelty: 0.5,
                ram_features: Default::default(),
                frame_embedding: Vec::new(),
                recent_inputs: None,
                parent_burst: Some(ProvenancedBurst {
                    burst: None,
                    provenance: None,
                }),
                sibling_bursts: vec![ScoredBurst {
                    burst: None,
                    score_delta: 0.75,
                }],
            }),
            k: 1,
            length_hint: 8,
            seed: 99,
            model: ModelKind::Pad as i32,
            config_overrides_yaml: Vec::new(),
        };
        let health = HealthResponse {
            status: health_response::Status::Serving as i32,
            synth_version: "test".to_owned(),
            loaded_packs: vec!["pack-a".to_owned()],
            loaded_experiments: vec!["exp-a".to_owned()],
            policy_endpoint_up: false,
            policy_deterministic: true,
            mining_in_progress: false,
        };
        let mine = MineMacrosRequest {
            experiment_id: "exp-a".to_owned(),
            paths: Vec::new(),
            params: None,
        };
        let mined = MineMacrosResponse {
            macro_pack_yaml: Vec::new(),
            pack_id: String::new(),
            stats: Vec::new(),
            paths_used: 0,
            tokens_scanned: 0,
        };

        assert_eq!(crate::inputsynth::v1::BURST_FORMAT_VERSION, 1);
        assert_eq!(load.kind, DocumentKind::ExperimentConfig as i32);
        assert_eq!(propose.k, 1);
        assert_eq!(health.status, health_response::Status::Serving as i32);
        assert_eq!(mine.experiment_id, "exp-a");
        assert_eq!(mined.paths_used, 0);
        let _ = HealthRequest {};
    }
}
