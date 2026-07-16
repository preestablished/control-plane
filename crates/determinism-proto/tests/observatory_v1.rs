#![cfg(feature = "observatory")]

use determinism_proto::observatory::v1::{
    event_ingest_client::EventIngestClient,
    event_ingest_server::{EventIngest, EventIngestServer},
    EventBatch, EventEnvelope, PublishAck, Rejection, SourceService,
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

fn envelope(seq: u64) -> EventEnvelope {
    EventEnvelope {
        envelope_version: 1,
        ts_logical: 4_812,
        ts_wall_ns: 1_789_000_000_000_000_000,
        run_id: "run-7f3a".into(),
        source_service: SourceService::ExplorationOrchestrator as i32,
        event_type: "best-score-improved".into(),
        payload_version: 1,
        payload_json: br#"{"node_id":"93","score":0.84,"prev_best":0.81,"expansion_idx":4812}"#
            .to_vec(),
        seq,
        producer_id: "orchestratord-1789000000".into(),
    }
}

#[test]
fn generated_service_surface_is_available() {
    let _client: Option<EventIngestClient<tonic::transport::Channel>> = None;
    let _server: Option<EventIngestServer<NoopIngest>> = None;
    let _trait: Option<&dyn EventIngest<PublishEventsStream = PublishStream>> = None;
}

#[test]
fn event_envelope_all_ten_fields_round_trip() {
    round_trip(envelope(4_812));
}

#[test]
fn source_service_variants_carry_documented_values() {
    assert_eq!(SourceService::Unspecified as i32, 0);
    assert_eq!(SourceService::ExplorationOrchestrator as i32, 1);
    assert_eq!(SourceService::DeterminismHypervisor as i32, 2);
    assert_eq!(SourceService::StateScorer as i32, 3);
    assert_eq!(SourceService::ReplayRenderer as i32, 4);
    assert_eq!(SourceService::ControlPlane as i32, 5);
    assert_eq!(SourceService::GuestSdk as i32, 6);
}

#[test]
fn event_batch_round_trips() {
    round_trip(EventBatch {
        events: vec![envelope(1), envelope(2)],
    });
}

#[test]
fn publish_ack_with_rejections_round_trips() {
    round_trip(PublishAck {
        acked_seq: 4_812,
        rejections: vec![Rejection {
            seq: 4_810,
            reason: "payload not a JSON object".into(),
        }],
    });
}

type PublishStream = std::pin::Pin<
    Box<dyn tonic::codegen::tokio_stream::Stream<Item = Result<PublishAck, tonic::Status>> + Send>,
>;

struct NoopIngest;

#[tonic::async_trait]
impl EventIngest for NoopIngest {
    type PublishEventsStream = PublishStream;

    async fn publish_events(
        &self,
        _request: tonic::Request<tonic::Streaming<EventEnvelope>>,
    ) -> Result<tonic::Response<Self::PublishEventsStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("noop"))
    }

    async fn publish_events_bulk(
        &self,
        _request: tonic::Request<EventBatch>,
    ) -> Result<tonic::Response<PublishAck>, tonic::Status> {
        Err(tonic::Status::unimplemented("noop"))
    }
}
