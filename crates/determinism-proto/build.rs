use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SCORER");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_INPUTSYNTH");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_ORCHESTRATOR");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_CONTROLPLANE");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_OBSERVATORY");

    let include_scorer = std::env::var_os("CARGO_FEATURE_SCORER").is_some();
    let include_inputsynth = std::env::var_os("CARGO_FEATURE_INPUTSYNTH").is_some();
    let include_orchestrator = std::env::var_os("CARGO_FEATURE_ORCHESTRATOR").is_some();
    let include_controlplane = std::env::var_os("CARGO_FEATURE_CONTROLPLANE").is_some();
    let include_observatory = std::env::var_os("CARGO_FEATURE_OBSERVATORY").is_some();

    if !include_scorer
        && !include_inputsynth
        && !include_orchestrator
        && !include_controlplane
        && !include_observatory
    {
        return Ok(());
    }

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .ok_or("failed to derive repository root from CARGO_MANIFEST_DIR")?;
    let workspace_proto_root = repo_root.join("proto");
    let packaged_proto_root = manifest_dir.join("proto");
    let proto_root = if workspace_proto_root.exists() {
        if packaged_proto_root.exists() {
            assert_proto_copies_match(
                &workspace_proto_root,
                &packaged_proto_root,
                include_scorer,
                include_inputsynth,
                include_orchestrator,
                include_controlplane,
                include_observatory,
            )?;
        }
        workspace_proto_root
    } else {
        packaged_proto_root
    };

    let mut protos = Vec::new();
    if include_scorer {
        protos.push(proto_root.join("determinism/scorer/v1/scorer.proto"));
    }
    if include_inputsynth {
        protos.push(proto_root.join("determinism/inputsynth/v1/synthesizer.proto"));
    }
    if include_orchestrator {
        protos.push(proto_root.join("determinism/orchestrator/v1/orchestrator.proto"));
    }
    if include_controlplane {
        protos.push(proto_root.join("determinism/controlplane/v1/resources.proto"));
    }
    if include_observatory {
        protos.push(proto_root.join("determinism/observatory/v1/events.proto"));
    }

    println!("cargo:rerun-if-changed={}", proto_root.display());
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let mut prost = prost_build::Config::new();
    prost.protoc_executable(protoc);

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(prost, &protos, &[proto_root])?;

    Ok(())
}

fn assert_proto_copies_match(
    workspace_proto_root: &std::path::Path,
    packaged_proto_root: &std::path::Path,
    include_scorer: bool,
    include_inputsynth: bool,
    include_orchestrator: bool,
    include_controlplane: bool,
    include_observatory: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut relative_paths = Vec::new();
    if include_scorer {
        relative_paths.push("determinism/scorer/v1/scorer.proto");
    }
    if include_inputsynth {
        relative_paths.push("determinism/inputsynth/v1/synthesizer.proto");
    }
    if include_orchestrator {
        relative_paths.push("determinism/orchestrator/v1/orchestrator.proto");
    }
    if include_controlplane {
        relative_paths.push("determinism/controlplane/v1/resources.proto");
    }
    if include_observatory {
        relative_paths.push("determinism/observatory/v1/events.proto");
    }

    for relative_path in relative_paths {
        let workspace_proto = workspace_proto_root.join(relative_path);
        let packaged_proto = packaged_proto_root.join(relative_path);
        if std::fs::read(&workspace_proto)? != std::fs::read(&packaged_proto)? {
            return Err(format!(
                "packaged proto copy is stale: {} differs from {}",
                packaged_proto.display(),
                workspace_proto.display()
            )
            .into());
        }
    }

    Ok(())
}
