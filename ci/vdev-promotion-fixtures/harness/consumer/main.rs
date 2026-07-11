use determinism_proto::scratch::v1::PromoteRequest;

fn main() {
    let request = PromoteRequest {
        value: "stable-seam".to_owned(),
        ..Default::default()
    };
    assert_eq!(request.value, "stable-seam");
}
