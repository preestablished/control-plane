#[cfg(feature = "inputsynth")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const INPUTSYNTH_PROTO: &str = "proto/determinism/inputsynth/v1/synthesizer.proto";

    println!("cargo:rerun-if-changed={INPUTSYNTH_PROTO}");

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", protoc);

    tonic_build::configure().compile_protos(&[INPUTSYNTH_PROTO], &["proto"])?;

    Ok(())
}

#[cfg(not(feature = "inputsynth"))]
fn main() {}
