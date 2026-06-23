#![cfg(feature = "inputsynth")]

use std::collections::HashMap;

use determinism_proto::inputsynth::v1::{
    burst, field_value, health_response,
    input_synthesizer_client::InputSynthesizerClient,
    input_synthesizer_server::{InputSynthesizer, InputSynthesizerServer},
    load_macro_pack_request, Burst, DegradedGenerator, DocumentKind, EventBurst, FieldValue,
    GeneratorKind, GrammarEvent, GrammarField, HealthRequest, HealthResponse, LoadMacroPackRequest,
    LoadMacroPackResponse, MacroProvenance, MineMacrosRequest, MineMacrosResponse, MinedMacroStats,
    MiningParams, ModelKind, MutationOp, MutationProvenance, NodeContext, PadBurst, PadSegment,
    PathSample, PolicyProvenance, ProposeBurstsRequest, ProposeBurstsResponse, Provenance,
    ProvenancedBurst, ScoredBurst, BURST_FORMAT_VERSION,
};
use prost::Message;

fn round_trip<T>(message: T)
where
    T: Message + Default + PartialEq + core::fmt::Debug,
{
    let bytes = message.encode_to_vec();
    let decoded = T::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded, message);
}

#[test]
fn generated_service_surface_is_available() {
    let _client: Option<InputSynthesizerClient<tonic::transport::Channel>> = None;
    let _server: Option<InputSynthesizerServer<NoopSynth>> = None;
    let _trait: Option<&dyn InputSynthesizer> = None;
    let _body = burst::Body::Pad(PadBurst::default());
    let _value = field_value::Value::EnumVal(String::new());
    let _source = load_macro_pack_request::Source::DocumentYaml(Vec::new());
    assert_eq!(BURST_FORMAT_VERSION, 1);
}

#[test]
fn propose_request_and_context_round_trip() {
    round_trip(ProposeBurstsRequest {
        experiment_id: "exp-a".into(),
        node_context: Some(node_context()),
        k: 8,
        length_hint: 120,
        seed: 99,
        model: ModelKind::Pad as i32,
        config_overrides_yaml: b"burst_len: 120".to_vec(),
    });
    round_trip(node_context());
}

#[test]
fn propose_response_and_provenanced_bursts_round_trip() {
    let burst = provenanced_burst();
    round_trip(ProvenancedBurst {
        burst: burst.burst.clone(),
        provenance: burst.provenance.clone(),
    });
    round_trip(ProposeBurstsResponse {
        bursts: vec![burst],
        config_fingerprint: vec![1; 32],
        synth_version: "0.1.0".into(),
        seed: 99,
        degraded: vec![DegradedGenerator {
            generator: GeneratorKind::Policy as i32,
            reason: "policy_endpoint_down".into(),
        }],
    });
    round_trip(DegradedGenerator {
        generator: GeneratorKind::Macro as i32,
        reason: "no_macros_loaded".into(),
    });
}

#[test]
fn pad_and_event_bursts_round_trip() {
    round_trip(pad_burst());
    round_trip(event_burst());
    round_trip(Burst {
        format_version: BURST_FORMAT_VERSION,
        burst_id: vec![2; 32],
        body: Some(burst::Body::Pad(PadBurst {
            segments: vec![PadSegment {
                buttons: 0b101,
                hold_frames: 12,
            }],
            button_alphabet: "console16-12btn-v1".into(),
        })),
    });
    round_trip(Burst {
        format_version: BURST_FORMAT_VERSION,
        burst_id: vec![3; 32],
        body: Some(burst::Body::Event(EventBurst {
            events: vec![GrammarEvent {
                event_type: "packet".into(),
                at_offset_ns: 100,
                fields: vec![GrammarField {
                    name: "kind".into(),
                    value: Some(FieldValue {
                        value: Some(field_value::Value::EnumVal("OPEN".into())),
                    }),
                }],
                payload: vec![1, 2, 3],
            }],
            grammar_id: "grammar-1".into(),
        })),
    });
}

#[test]
fn field_value_variants_round_trip() {
    for value in [
        field_value::Value::IntVal(-7),
        field_value::Value::EnumVal("LEFT".into()),
        field_value::Value::DurNs(16_666_667),
        field_value::Value::BytesVal(vec![1, 2, 3]),
    ] {
        round_trip(FieldValue { value: Some(value) });
    }
}

#[test]
fn scored_burst_and_provenance_submessages_round_trip() {
    round_trip(ScoredBurst {
        burst: Some(provenanced_burst()),
        score_delta: 1.5,
    });
    round_trip(macro_provenance());
    round_trip(mutation_provenance());
    round_trip(policy_provenance());
    round_trip(provenance());
}

#[test]
fn macro_pack_and_mining_messages_round_trip() {
    for source in [
        load_macro_pack_request::Source::DocumentYaml(b"kind: macro_pack".to_vec()),
        load_macro_pack_request::Source::ArtifactRef("artifact:pack".into()),
    ] {
        round_trip(LoadMacroPackRequest {
            source: Some(source),
            kind: DocumentKind::MacroPack as i32,
        });
    }

    round_trip(LoadMacroPackResponse {
        document_id: "pack-1".into(),
        items_loaded: 4,
        warnings: vec!["shadowed macro".into()],
    });

    let path = PathSample {
        expansions: vec![ScoredBurst {
            burst: Some(provenanced_burst()),
            score_delta: 2.0,
        }],
        terminal_score: 10.0,
    };
    round_trip(path.clone());

    let params = MiningParams {
        min_support: 5,
        min_paths: 3,
        max_len_tokens: 24,
        max_macros: 32,
        containment_alpha: 0.8,
        dedup_edit_dist: 0.2,
    };
    round_trip(params.clone());
    round_trip(MineMacrosRequest {
        experiment_id: "exp-a".into(),
        paths: vec![path],
        params: Some(params),
    });

    let stats = MinedMacroStats {
        name: "mined-a3f2dd-007".into(),
        support: 23,
        paths: 9,
        lift: 4.31,
        score: 18.2,
        len_tokens: 6,
    };
    round_trip(stats.clone());
    round_trip(MineMacrosResponse {
        macro_pack_yaml: b"version: 1".to_vec(),
        pack_id: "pack-1".into(),
        stats: vec![stats],
        paths_used: 3,
        tokens_scanned: 99,
    });
}

#[test]
fn health_response_round_trips() {
    round_trip(HealthRequest {});
    round_trip(HealthResponse {
        status: health_response::Status::Degraded as i32,
        synth_version: "0.1.0".into(),
        loaded_packs: vec!["pack-1".into()],
        loaded_experiments: vec!["exp-a".into()],
        policy_endpoint_up: true,
        policy_deterministic: false,
        mining_in_progress: true,
    });
}

#[test]
fn enum_numeric_values_are_stable() {
    assert_eq!(ModelKind::Unspecified as i32, 0);
    assert_eq!(ModelKind::Pad as i32, 1);
    assert_eq!(ModelKind::EventGrammar as i32, 2);

    assert_eq!(GeneratorKind::Unspecified as i32, 0);
    assert_eq!(GeneratorKind::WeightedRandom as i32, 1);
    assert_eq!(GeneratorKind::Macro as i32, 2);
    assert_eq!(GeneratorKind::Mutation as i32, 3);
    assert_eq!(GeneratorKind::Policy as i32, 4);

    assert_eq!(DocumentKind::Unspecified as i32, 0);
    assert_eq!(DocumentKind::MacroPack as i32, 1);
    assert_eq!(DocumentKind::ExperimentConfig as i32, 2);
    assert_eq!(DocumentKind::EventGrammar as i32, 3);

    assert_eq!(health_response::Status::Unspecified as i32, 0);
    assert_eq!(health_response::Status::Serving as i32, 1);
    assert_eq!(health_response::Status::Degraded as i32, 2);
    assert_eq!(health_response::Status::NotServing as i32, 3);
}

fn pad_burst() -> PadBurst {
    PadBurst {
        segments: vec![PadSegment {
            buttons: 1,
            hold_frames: 8,
        }],
        button_alphabet: "console16-12btn-v1".into(),
    }
}

fn event_burst() -> EventBurst {
    EventBurst {
        events: vec![GrammarEvent {
            event_type: "packet".into(),
            at_offset_ns: 42,
            fields: vec![GrammarField {
                name: "opcode".into(),
                value: Some(FieldValue {
                    value: Some(field_value::Value::IntVal(7)),
                }),
            }],
            payload: vec![9],
        }],
        grammar_id: "grammar-1".into(),
    }
}

fn burst() -> Burst {
    Burst {
        format_version: BURST_FORMAT_VERSION,
        burst_id: vec![4; 32],
        body: Some(burst::Body::Pad(pad_burst())),
    }
}

fn provenanced_burst() -> ProvenancedBurst {
    ProvenancedBurst {
        burst: Some(burst()),
        provenance: Some(provenance()),
    }
}

fn node_context() -> NodeContext {
    NodeContext {
        node_id: "42".into(),
        snapshot_ref: "snap-ref".into(),
        depth: 3,
        node_score: 12.0,
        novelty: 0.25,
        ram_features: HashMap::from([("player_x".into(), 100.0)]),
        frame_embedding: vec![0.1, 0.2],
        recent_inputs: Some(burst()),
        parent_burst: Some(provenanced_burst()),
        sibling_bursts: vec![ScoredBurst {
            burst: Some(provenanced_burst()),
            score_delta: 1.0,
        }],
    }
}

fn provenance() -> Provenance {
    Provenance {
        generator: GeneratorKind::Macro as i32,
        slot: 1,
        rng_stream: "slot/1/macro".into(),
        config_fingerprint: vec![5; 32],
        fallback_from: GeneratorKind::Unspecified as i32,
        r#macro: Some(macro_provenance()),
        mutation: Some(mutation_provenance()),
        policy: Some(policy_provenance()),
    }
}

fn macro_provenance() -> MacroProvenance {
    MacroProvenance {
        pack_id: "pack-1".into(),
        macro_name: "long-jump".into(),
        param_bindings: HashMap::from([("dir".into(), "right".into())]),
        macro_frames: 36,
        tail_frames: 4,
        chain_index: 0,
    }
}

fn mutation_provenance() -> MutationProvenance {
    MutationProvenance {
        base_burst_id: vec![6; 32],
        donor_burst_id: vec![7; 32],
        base_was_sibling: true,
        ops: vec![MutationOp {
            op: "splice".into(),
            args: HashMap::from([("cut".into(), "3".into())]),
        }],
        post_clamp: true,
    }
}

fn policy_provenance() -> PolicyProvenance {
    PolicyProvenance {
        model_id: "policy-a".into(),
        model_version: "2026.06".into(),
        temperature: 0.7,
        server_attested_deterministic: true,
    }
}

#[derive(Debug, Default)]
struct NoopSynth;

#[tonic::async_trait]
impl InputSynthesizer for NoopSynth {
    async fn propose_bursts(
        &self,
        _request: tonic::Request<ProposeBurstsRequest>,
    ) -> Result<tonic::Response<ProposeBurstsResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn load_macro_pack(
        &self,
        _request: tonic::Request<LoadMacroPackRequest>,
    ) -> Result<tonic::Response<LoadMacroPackResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn mine_macros(
        &self,
        _request: tonic::Request<MineMacrosRequest>,
    ) -> Result<tonic::Response<MineMacrosResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn health(
        &self,
        _request: tonic::Request<HealthRequest>,
    ) -> Result<tonic::Response<HealthResponse>, tonic::Status> {
        unimplemented!()
    }
}
