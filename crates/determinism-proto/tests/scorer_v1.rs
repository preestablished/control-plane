#![cfg(feature = "scorer")]

use determinism_proto::scorer::v1::{
    load_feature_map_request, load_scoring_program_request, state_input,
    state_scorer_client::StateScorerClient,
    state_scorer_server::{StateScorer, StateScorerServer},
    ArchiveUpdateMode, CheckpointArchiveRequest, CheckpointArchiveResponse, CommittedState,
    ComponentScore, ExtractRange, FrameSpec, FramebufferMeta, GpuInfo, HealthRequest,
    HealthResponse, ItemError, ItemErrorKind, LatencyHistogram, LoadFeatureMapRequest,
    LoadFeatureMapResponse, LoadScoringProgramRequest, LoadScoringProgramResponse, NoveltyDetail,
    PixelFormat, ReplayCommitsRequest, ReplayCommitsResponse, RestoreArchiveRequest,
    RestoreArchiveResponse, ScoreBatchRequest, ScoreBatchResponse, ScoreResult, ServingStatus,
    StateInput, StatsRequest, StatsResponse,
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
    let _client: Option<StateScorerClient<tonic::transport::Channel>> = None;
    let _server: Option<StateScorerServer<NoopScorer>> = None;
    let _trait: Option<&dyn StateScorer> = None;
    let _fb = state_input::Framebuffer::FbRaw(Vec::new());
    let _map_source = load_feature_map_request::Source::InlineYaml(Vec::new());
    let _program_source = load_scoring_program_request::Source::InlineYaml(Vec::new());
}

#[test]
fn score_batch_request_round_trips() {
    round_trip(ScoreBatchRequest {
        experiment_id: "exp-a".into(),
        states: vec![state_input_with(state_input::Framebuffer::FbLz4(vec![
            1, 2, 3,
        ]))],
        archive_update: ArchiveUpdateMode::ScoreAndInsert as i32,
        client_batch_id: "batch-7".into(),
        return_decoded: true,
    });
}

#[test]
fn score_batch_response_round_trips() {
    round_trip(ScoreBatchResponse {
        client_batch_id: "batch-7".into(),
        archive_seq: 9,
        results: vec![ScoreResult {
            node_ref: "node-a".into(),
            error: Some(ItemError {
                kind: ItemErrorKind::FbRefUnsupported as i32,
                detail: "blob resolver unavailable".into(),
            }),
            progress_score: 12.5,
            novelty_score: 0.75,
            state_hash: vec![4; 32],
            goal_hit: true,
            duplicate: false,
            component_breakdown: vec![ComponentScore {
                name: "stage_1".into(),
                value: 10.0,
                unlocked: true,
            }],
            novelty_detail: Some(NoveltyDetail {
                count_novelty: 0.5,
                cell_key: 42,
                cell_count: 3,
                visual_novelty: 0.25,
                phash: 77,
                phash_min_hamming: 5,
                rnd_error: 0.1,
                knn_distance: 0.2,
            }),
            stage: 1,
            prune: false,
            decoded: vec![1.0, 2.0],
        }],
    });
}

#[test]
fn state_input_framebuffer_variants_round_trip() {
    for framebuffer in [
        state_input::Framebuffer::FbLz4(vec![1]),
        state_input::Framebuffer::FbRaw(vec![2]),
        state_input::Framebuffer::FbBlobRef("blob:abc".into()),
    ] {
        round_trip(state_input_with(framebuffer));
    }
}

#[test]
fn load_requests_and_responses_round_trip() {
    for source in [
        load_feature_map_request::Source::InlineYaml(b"kind: feature-map".to_vec()),
        load_feature_map_request::Source::ArtifactRef("artifact:feature-map".into()),
    ] {
        round_trip(LoadFeatureMapRequest {
            experiment_id: "exp-a".into(),
            source: Some(source),
            layout: Some(determinism_proto::scorer::v1::CompiledLayout {
                ranges: vec![ExtractRange {
                    region: "wram".into(),
                    layout_version: 1,
                    offset: 16,
                    len: 8,
                }],
            }),
            frame: Some(FrameSpec {
                width: 256,
                height: 224,
                stride: 1024,
                format: PixelFormat::Xrgb8888 as i32,
            }),
            rebin: true,
        });
    }

    for source in [
        load_scoring_program_request::Source::InlineYaml(b"kind: scoring-program".to_vec()),
        load_scoring_program_request::Source::ArtifactRef("artifact:program".into()),
    ] {
        round_trip(LoadScoringProgramRequest {
            experiment_id: "exp-a".into(),
            source: Some(source),
        });
    }

    round_trip(LoadFeatureMapResponse {
        feature_map_hash: vec![1; 32],
        field_count: 5,
        feature_bytes_len: 8,
        warnings: vec!["warn".into()],
    });
    round_trip(LoadScoringProgramResponse {
        program_hash: vec![2; 32],
        component_names: vec!["stage".into()],
        goal_expr: "flag == 1".into(),
        warnings: vec!["warn".into()],
        stage_names: vec!["stage".into()],
    });
}

#[test]
fn archive_replay_stats_and_health_round_trip() {
    round_trip(CheckpointArchiveRequest {
        experiment_id: "exp-a".into(),
        checkpoint_id: "ckpt-1".into(),
    });
    round_trip(CheckpointArchiveResponse {
        archive_ref: "scar:blake3:abc".into(),
        archive_hash: vec![3; 32],
        archive_seq: 10,
        cell_count: 11,
        phash_count: 12,
        embedding_count: 13,
        blob_bytes: 14,
    });
    round_trip(RestoreArchiveRequest {
        experiment_id: "exp-a".into(),
        checkpoint_id: "ckpt-1".into(),
        archive_ref: "scar:blake3:abc".into(),
    });
    round_trip(RestoreArchiveResponse {
        archive_seq: 10,
        cell_count: 11,
        phash_count: 12,
        embedding_count: 13,
        bound_feature_map_hash: vec![4; 32],
        bound_scoring_program_hash: vec![5; 32],
    });
    round_trip(ReplayCommitsRequest {
        experiment_id: "exp-a".into(),
        states: vec![CommittedState {
            state_hash: vec![6; 32],
            cell_key: 99,
        }],
    });
    round_trip(ReplayCommitsResponse {
        applied: 1,
        skipped: 2,
    });
    round_trip(StatsRequest {
        experiment_id: "exp-a".into(),
    });
    round_trip(StatsResponse {
        batches_total: 1,
        states_total: 2,
        item_errors_total: 3,
        dedup_rate: 0.4,
        cell_count: 5,
        mean_count_novelty_last_1k: 0.6,
        phash_count: 7,
        embedding_count: 8,
        archive_seq: 9,
        goal_hits_total: 10,
        batch_latency: Some(LatencyHistogram {
            p50_us: 11,
            p90_us: 12,
            p99_us: 13,
            max_us: 14,
        }),
        gpu: Some(GpuInfo {
            present: true,
            name: "cpu".into(),
            mem_used: 15,
            backend: "cpu".into(),
        }),
        loaded_feature_map_hash: vec![7; 32],
        loaded_scoring_program_hash: vec![8; 32],
    });
    round_trip(HealthRequest {});
    round_trip(HealthResponse {
        status: ServingStatus::Serving as i32,
        version: "0.1.0".into(),
    });
}

#[test]
fn enum_numeric_values_are_stable() {
    assert_eq!(ArchiveUpdateMode::Unspecified as i32, 0);
    assert_eq!(ArchiveUpdateMode::ScoreAndInsert as i32, 1);
    assert_eq!(ArchiveUpdateMode::ScoreOnly as i32, 2);

    assert_eq!(PixelFormat::Unspecified as i32, 0);
    assert_eq!(PixelFormat::Rgb888 as i32, 1);
    assert_eq!(PixelFormat::Rgb555Le as i32, 2);
    assert_eq!(PixelFormat::Gray8 as i32, 3);
    assert_eq!(PixelFormat::Xrgb8888 as i32, 4);

    assert_eq!(ItemErrorKind::Unspecified as i32, 0);
    assert_eq!(ItemErrorKind::FeatureLenMismatch as i32, 1);
    assert_eq!(ItemErrorKind::DecodeFailed as i32, 2);
    assert_eq!(ItemErrorKind::FbMetaMismatch as i32, 3);
    assert_eq!(ItemErrorKind::FbDecompressFailed as i32, 4);
    assert_eq!(ItemErrorKind::FbRefUnsupported as i32, 5);

    assert_eq!(ServingStatus::Unspecified as i32, 0);
    assert_eq!(ServingStatus::Serving as i32, 1);
    assert_eq!(ServingStatus::NotServing as i32, 2);
}

fn state_input_with(framebuffer: state_input::Framebuffer) -> StateInput {
    StateInput {
        node_ref: "node-a".into(),
        feature_bytes: vec![9, 8, 7],
        framebuffer: Some(framebuffer),
        fb_meta: Some(FramebufferMeta {
            width: 256,
            height: 224,
            format: PixelFormat::Xrgb8888 as i32,
            uncompressed_len: 229_376,
        }),
    }
}

#[derive(Debug, Default)]
struct NoopScorer;

#[tonic::async_trait]
impl StateScorer for NoopScorer {
    async fn score_batch(
        &self,
        _request: tonic::Request<ScoreBatchRequest>,
    ) -> Result<tonic::Response<ScoreBatchResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn load_feature_map(
        &self,
        _request: tonic::Request<LoadFeatureMapRequest>,
    ) -> Result<tonic::Response<LoadFeatureMapResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn load_scoring_program(
        &self,
        _request: tonic::Request<LoadScoringProgramRequest>,
    ) -> Result<tonic::Response<LoadScoringProgramResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn checkpoint_archive(
        &self,
        _request: tonic::Request<CheckpointArchiveRequest>,
    ) -> Result<tonic::Response<CheckpointArchiveResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn restore_archive(
        &self,
        _request: tonic::Request<RestoreArchiveRequest>,
    ) -> Result<tonic::Response<RestoreArchiveResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn replay_commits(
        &self,
        _request: tonic::Request<ReplayCommitsRequest>,
    ) -> Result<tonic::Response<ReplayCommitsResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn stats(
        &self,
        _request: tonic::Request<StatsRequest>,
    ) -> Result<tonic::Response<StatsResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn health(
        &self,
        _request: tonic::Request<HealthRequest>,
    ) -> Result<tonic::Response<HealthResponse>, tonic::Status> {
        unimplemented!()
    }
}
