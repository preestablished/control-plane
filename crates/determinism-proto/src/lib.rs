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
        include!(concat!(env!("OUT_DIR"), "/determinism.controlplane.v1.rs"));
    }
}

#[cfg(feature = "orchestrator")]
pub mod orchestrator {
    pub mod v1 {
        tonic::include_proto!("determinism.orchestrator.v1");
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
        tonic::include_proto!("determinism.observatory.v1");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_proto_tag_matching_crate_version() {
        assert_eq!(
            crate::PROTO_VERSION,
            concat!("proto-v", env!("CARGO_PKG_VERSION"))
        );
    }

    #[cfg(feature = "controlplane")]
    #[test]
    fn controlplane_experiment_spec_generated_wire_type_round_trips() {
        use crate::controlplane::v1::{
            Budgets, BurstConfig, CheckpointConfig, ExperimentSpec, LadderConfig, OnGoal,
            PlateauConfig, PolicyKind, PruneAction, SchedMode, SchedulingConfig, SelectionConfig,
            StagedConfig,
        };
        use prost::Message;

        let spec = ExperimentSpec {
            version: 1,
            seed: 42,
            workload_image_ref: "image:workload".to_owned(),
            feature_map_ref: "feature:map".to_owned(),
            scoring_program_ref: "score:program".to_owned(),
            synth_config_ref: "synth:config".to_owned(),
            macro_pack_refs: vec!["macro:pack".to_owned()],
            budgets: Some(Budgets {
                max_nodes: 1_000,
                max_wall_clock_s: 86_400,
                max_guest_instructions: 2_000,
                max_expansions: 500,
            }),
            selection: Some(SelectionConfig {
                policy: PolicyKind::Staged as i32,
                alpha: 1.0,
                beta: 0.5,
                gamma: 0.3,
                delta: 0.1,
                temperature: 1.25,
                ucb_c: 1.4,
                staged: Some(StagedConfig {
                    inner: PolicyKind::Ucb as i32,
                    epsilon_regress: 0.05,
                }),
                max_visits_per_node: 64,
                exhaust_after_dup_expansions: 8,
            }),
            burst: Some(BurstConfig {
                k_per_expansion: 16,
                base_burst_len_frames: 120,
                max_burst_len_frames: 600,
                max_guest_instructions_per_job: 10_000,
            }),
            plateau: Some(PlateauConfig {
                window_n: 200,
                epsilon_s: 0.001,
                ladder: Some(LadderConfig {
                    burst_len_factor: 1.5,
                    temp_factor: 1.75,
                    macro_weight_hot: 0.5,
                    backtrack_kappa: 1.0,
                    backtrack_depth_quantile: 0.5,
                    radius_factor: 2.0,
                    max_level: 4,
                }),
            }),
            scheduling: Some(SchedulingConfig {
                mode: SchedMode::Deterministic as i32,
                max_inflight_batches: 1,
                job_timeout_s: 120,
                retry_max: 3,
                retry_backoff_ms: 250,
                hypervisor_endpoints: vec!["http://127.0.0.1:9000".to_owned()],
                allow_class_mismatch: false,
            }),
            checkpoint: Some(CheckpointConfig {
                every_commits: 64,
                every_seconds: 30,
            }),
            prune_action: PruneAction::Drop as i32,
            on_goal: OnGoal::Continue as i32,
            decoded_features: vec!["stage".to_owned()],
        };

        let bytes = spec.encode_to_vec();
        let decoded = ExperimentSpec::decode(bytes.as_slice()).unwrap();
        assert_eq!(decoded, spec);
        assert_eq!(spec.seed, 42);
        assert_eq!(
            spec.selection
                .as_ref()
                .and_then(|selection| selection.staged.as_ref())
                .map(|staged| staged.inner),
            Some(PolicyKind::Ucb as i32)
        );
        assert_eq!(PruneAction::Exhausted as i32, 0);
        assert_eq!(PruneAction::Drop as i32, 1);
        assert_eq!(OnGoal::Stop as i32, 0);
        assert_eq!(OnGoal::Continue as i32, 1);
        assert_eq!(PolicyKind::Softmax as i32, 0);
        assert_eq!(PolicyKind::Ucb as i32, 1);
        assert_eq!(PolicyKind::Staged as i32, 2);
        assert_eq!(SchedMode::Fast as i32, 0);
        assert_eq!(SchedMode::Deterministic as i32, 1);
    }

    #[cfg(feature = "controlplane")]
    #[test]
    fn controlplane_experiment_spec_seed_default_construction_still_compiles() {
        let spec = crate::controlplane::v1::ExperimentSpec {
            seed: 1,
            ..Default::default()
        };

        assert_eq!(spec.seed, 1);
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

    #[cfg(feature = "orchestrator")]
    #[test]
    fn orchestrator_facade_exposes_required_generated_symbols() {
        use crate::orchestrator::v1::{
            exploration_orchestrator_client::ExplorationOrchestratorClient, progress_event,
            Budgets, ExperimentConfig, ExperimentState, ExperimentStatus, GoalReached, PolicyKind,
            ProgressEvent, SchedMode, SelectionConfig, StartExperimentRequest,
        };

        let _client_type =
            std::mem::size_of::<ExplorationOrchestratorClient<tonic::transport::Channel>>();
        let start = StartExperimentRequest {
            experiment_id: "exp-a".to_owned(),
            config: Some(ExperimentConfig {
                version: 1,
                seed: 42,
                budgets: Some(Budgets {
                    max_nodes: 1_000,
                    ..Default::default()
                }),
                selection: Some(SelectionConfig {
                    policy: PolicyKind::Softmax as i32,
                    temperature: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            resume_if_exists: true,
            run_id: String::new(),
        };
        let event = ProgressEvent {
            at: None,
            status: Some(ExperimentStatus {
                experiment_id: "exp-a".to_owned(),
                state: ExperimentState::Running as i32,
                ..Default::default()
            }),
            edge: Some(progress_event::Edge::Goal(GoalReached {
                node_id: 7,
                snapshot_ref: vec![0u8; 32],
                score: 12.5,
                depth: 3,
            })),
        };

        assert_eq!(start.config.as_ref().map(|c| c.seed), Some(42));
        assert!(matches!(
            event.edge,
            Some(progress_event::Edge::Goal(GoalReached { node_id: 7, .. }))
        ));
        assert_eq!(
            ExperimentState::try_from(ExperimentState::GoalReached as i32),
            Ok(ExperimentState::GoalReached)
        );
        assert_eq!(SchedMode::Fast as i32, 0);
    }
}
