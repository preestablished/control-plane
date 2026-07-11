#![forbid(unsafe_code)]

use prost::Message;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

struct Side {
    root: PathBuf,
    file: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("descriptor comparison failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (owner, control) = parse_args()?;
    let mut owner_set = compile(&owner)?;
    let mut control_set = compile(&control)?;

    canonicalize(&mut owner_set, &owner.file, &control.file)?;
    canonicalize(&mut control_set, &control.file, &control.file)?;

    if owner_set != control_set {
        report_diff(&owner_set, &control_set);
        return Err("canonical descriptor sets differ".into());
    }

    println!(
        "descriptor sets are equivalent: {} == {}",
        owner.root.join(&owner.file).display(),
        control.root.join(&control.file).display()
    );
    Ok(())
}

fn parse_args() -> Result<(Side, Side), Box<dyn std::error::Error>> {
    let mut owner_root = None;
    let mut owner_file = None;
    let mut control_root = None;
    let mut control_file = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--owner-root" => owner_root = Some(PathBuf::from(value)),
            "--owner-file" => owner_file = Some(value),
            "--control-root" => control_root = Some(PathBuf::from(value)),
            "--control-file" => control_file = Some(value),
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }

    Ok((
        Side {
            root: owner_root.ok_or("--owner-root is required")?,
            file: owner_file.ok_or("--owner-file is required")?,
        },
        Side {
            root: control_root.ok_or("--control-root is required")?,
            file: control_file.ok_or("--control-file is required")?,
        },
    ))
}

fn compile(side: &Side) -> Result<FileDescriptorSet, Box<dyn std::error::Error>> {
    if Path::new(&side.file).is_absolute() {
        return Err("proto file arguments must be relative to their include roots".into());
    }
    let output_dir = tempfile::tempdir()?;
    let descriptor_path = output_dir.path().join("descriptor.pb");
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let root = side.root.canonicalize()?;
    let output = Command::new(protoc)
        .current_dir(&root)
        .arg(format!("--proto_path={}", root.display()))
        .arg("--include_imports")
        .arg("--include_source_info")
        .arg(format!(
            "--descriptor_set_out={}",
            descriptor_path.display()
        ))
        .arg(&side.file)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "protoc failed for {}: {}",
            side.root.join(&side.file).display(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(FileDescriptorSet::decode(
        std::fs::read(descriptor_path)?.as_slice(),
    )?)
}

fn canonicalize(
    set: &mut FileDescriptorSet,
    source_root_file: &str,
    canonical_root_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut names = BTreeSet::new();
    let mut found_root = false;
    for file in &mut set.file {
        file.source_code_info = None;
        if file.name.as_deref() == Some(source_root_file) {
            file.name = Some(canonical_root_file.to_owned());
            found_root = true;
        }
        for dependency in &mut file.dependency {
            if dependency == source_root_file {
                *dependency = canonical_root_file.to_owned();
            }
        }
        let name = file.name.clone().ok_or("descriptor file has no name")?;
        if !names.insert(name.clone()) {
            return Err(format!("duplicate descriptor file name: {name}").into());
        }
    }
    if !found_root {
        return Err(format!("root descriptor not found: {source_root_file}").into());
    }
    set.file.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(())
}

fn report_diff(owner: &FileDescriptorSet, control: &FileDescriptorSet) {
    let owner_files = owner
        .file
        .iter()
        .map(summary)
        .collect::<Vec<_>>()
        .join("\n");
    let control_files = control
        .file
        .iter()
        .map(summary)
        .collect::<Vec<_>>()
        .join("\n");
    eprintln!("owner descriptors:\n{owner_files}");
    eprintln!("control-plane descriptors:\n{control_files}");

    for (owner_file, control_file) in owner.file.iter().zip(&control.file) {
        if owner_file != control_file {
            eprintln!("first differing owner file:\n{owner_file:#?}");
            eprintln!("first differing control file:\n{control_file:#?}");
            break;
        }
    }
}

fn summary(file: &FileDescriptorProto) -> String {
    format!(
        "- {} package={} messages={} enums={} services={} dependencies={:?}",
        file.name.as_deref().unwrap_or("<unnamed>"),
        file.package.as_deref().unwrap_or("<none>"),
        file.message_type.len(),
        file.enum_type.len(),
        file.service.len(),
        file.dependency
    )
}
