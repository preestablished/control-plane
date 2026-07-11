use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SCRATCH");
    if std::env::var_os("CARGO_FEATURE_SCRATCH").is_none() {
        return Ok(());
    }
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("proto");
    let proto = root.join("determinism/scratch/v1/scratch.proto");
    println!("cargo:rerun-if-changed={}", proto.display());
    let mut prost = prost_build::Config::new();
    prost.protoc_executable(protoc_bin_vendored::protoc_bin_path()?);
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(prost, &[proto], &[root])?;
    Ok(())
}
