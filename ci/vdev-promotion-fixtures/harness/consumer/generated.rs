use determinism_proto::scratch::v1::{
    scratch_service_client::ScratchServiceClient, PromoteRequest,
};
use prost::Message;

fn main() {
    let request = PromoteRequest {
        value: "generated".to_owned(),
        ..Default::default()
    };
    let bytes = request.encode_to_vec();
    let decoded = PromoteRequest::decode(bytes.as_slice()).expect("round trip");
    assert_eq!(decoded.value, "generated");
    let _ = std::mem::size_of::<ScratchServiceClient<tonic::transport::Channel>>();
}
