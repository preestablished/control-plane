fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/determinism/inputsynth/v1/synthesizer.proto");

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", protoc);

    tonic_build::configure().compile_protos(
        &["../../proto/determinism/inputsynth/v1/synthesizer.proto"],
        &["../../proto"],
    )?;

    Ok(())
}
