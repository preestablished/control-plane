use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use prost::Message;
use prost_types::{
    descriptor_proto, enum_descriptor_proto,
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, EnumOptions, EnumValueOptions, FieldDescriptorProto,
    FieldOptions, FileDescriptorProto, FileDescriptorSet, MessageOptions, OneofDescriptorProto,
    OneofOptions,
};

const CONTROLPLANE_PACKAGE: &str = "determinism.controlplane.v1";
const ORCHESTRATOR_PACKAGE: &str = "determinism.orchestrator.v1";

const MESSAGE_CLOSURE: &[(&str, &str)] = &[
    ("ExperimentSpec", "ExperimentConfig"),
    ("Budgets", "Budgets"),
    ("SelectionConfig", "SelectionConfig"),
    ("StagedConfig", "StagedConfig"),
    ("BurstConfig", "BurstConfig"),
    ("PlateauConfig", "PlateauConfig"),
    ("LadderConfig", "LadderConfig"),
    ("SchedulingConfig", "SchedulingConfig"),
    ("CheckpointConfig", "CheckpointConfig"),
];

const ENUM_CLOSURE: &[(&str, &str)] = &[
    ("PruneAction", "PruneAction"),
    ("OnGoal", "OnGoal"),
    ("PolicyKind", "PolicyKind"),
    ("SchedMode", "SchedMode"),
];

#[test]
fn experiment_spec_mirrors_orchestrator_experiment_config() {
    let descriptors = descriptor_set();
    let index = DescriptorIndex::new(&descriptors);

    let expected_messages: BTreeSet<_> = MESSAGE_CLOSURE
        .iter()
        .map(|(controlplane, _)| controlplane.to_string())
        .collect();
    let actual_messages = index.message_names(CONTROLPLANE_PACKAGE);
    assert_eq!(
        actual_messages, expected_messages,
        "controlplane mirror contains missing or extra messages"
    );

    let expected_enums: BTreeSet<_> = ENUM_CLOSURE
        .iter()
        .map(|(controlplane, _)| controlplane.to_string())
        .collect();
    let actual_enums = index.enum_names(CONTROLPLANE_PACKAGE);
    assert_eq!(
        actual_enums, expected_enums,
        "controlplane mirror contains missing or extra enums"
    );

    for (controlplane_name, orchestrator_name) in MESSAGE_CLOSURE {
        let controlplane = index
            .message(CONTROLPLANE_PACKAGE, controlplane_name)
            .unwrap_or_else(|| panic!("missing controlplane message {controlplane_name}"));
        let orchestrator = index
            .message(ORCHESTRATOR_PACKAGE, orchestrator_name)
            .unwrap_or_else(|| panic!("missing orchestrator message {orchestrator_name}"));

        assert_eq!(
            normalize_message(controlplane),
            normalize_message(orchestrator),
            "message {controlplane_name} does not mirror {orchestrator_name}"
        );
    }

    for (controlplane_name, orchestrator_name) in ENUM_CLOSURE {
        let controlplane = index
            .enumeration(CONTROLPLANE_PACKAGE, controlplane_name)
            .unwrap_or_else(|| panic!("missing controlplane enum {controlplane_name}"));
        let orchestrator = index
            .enumeration(ORCHESTRATOR_PACKAGE, orchestrator_name)
            .unwrap_or_else(|| panic!("missing orchestrator enum {orchestrator_name}"));

        assert_eq!(
            normalize_enum(controlplane),
            normalize_enum(orchestrator),
            "enum {controlplane_name} does not mirror {orchestrator_name}"
        );
    }
}

fn descriptor_set() -> FileDescriptorSet {
    let repo_root = repo_root();
    let output_dir = tempfile::tempdir().expect("failed to create temporary descriptor directory");
    let descriptor_path = output_dir.path().join("mirror.pb");
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("failed to locate vendored protoc");

    let status = Command::new(protoc)
        .current_dir(&repo_root)
        .arg("--include_imports")
        .arg(format!(
            "--descriptor_set_out={}",
            descriptor_path.to_string_lossy()
        ))
        .arg("-I")
        .arg(repo_root.join("proto"))
        .arg("determinism/controlplane/v1/resources.proto")
        .arg("determinism/orchestrator/v1/orchestrator.proto")
        .status()
        .expect("failed to run protoc");
    assert!(status.success(), "protoc descriptor generation failed");

    let bytes = std::fs::read(descriptor_path).expect("failed to read descriptor set");
    FileDescriptorSet::decode(bytes.as_slice()).expect("failed to decode descriptor set")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("failed to derive repository root")
        .to_path_buf()
}

struct DescriptorIndex<'a> {
    files_by_package: BTreeMap<&'a str, &'a FileDescriptorProto>,
}

impl<'a> DescriptorIndex<'a> {
    fn new(descriptors: &'a FileDescriptorSet) -> Self {
        let files_by_package = descriptors
            .file
            .iter()
            .filter_map(|file| Some((file.package.as_deref()?, file)))
            .collect();

        Self { files_by_package }
    }

    fn message(&self, package: &str, name: &str) -> Option<&'a DescriptorProto> {
        self.files_by_package
            .get(package)?
            .message_type
            .iter()
            .find(|message| message.name.as_deref() == Some(name))
    }

    fn enumeration(&self, package: &str, name: &str) -> Option<&'a EnumDescriptorProto> {
        self.files_by_package
            .get(package)?
            .enum_type
            .iter()
            .find(|enumeration| enumeration.name.as_deref() == Some(name))
    }

    fn message_names(&self, package: &str) -> BTreeSet<String> {
        self.files_by_package
            .get(package)
            .into_iter()
            .flat_map(|file| &file.message_type)
            .filter_map(|message| message.name.clone())
            .collect()
    }

    fn enum_names(&self, package: &str) -> BTreeSet<String> {
        self.files_by_package
            .get(package)
            .into_iter()
            .flat_map(|file| &file.enum_type)
            .filter_map(|enumeration| enumeration.name.clone())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct NormalizedMessage {
    fields: Vec<NormalizedField>,
    nested_messages: Vec<NormalizedNestedMessage>,
    nested_enums: Vec<NormalizedNestedEnum>,
    oneofs: Vec<NormalizedOneof>,
    reserved_ranges: Vec<NormalizedReservedRange>,
    reserved_names: Vec<String>,
    options: Option<MessageOptions>,
}

#[derive(Debug, PartialEq)]
struct NormalizedNestedMessage {
    name: String,
    message: NormalizedMessage,
}

#[derive(Debug, PartialEq)]
struct NormalizedNestedEnum {
    name: String,
    enumeration: NormalizedEnum,
}

#[derive(Debug, PartialEq)]
struct NormalizedOneof {
    name: String,
    options: Option<OneofOptions>,
}

#[derive(Debug, PartialEq, Eq)]
struct NormalizedReservedRange {
    start: i32,
    end: i32,
}

#[derive(Debug, PartialEq)]
struct NormalizedField {
    name: String,
    number: i32,
    label: Option<Label>,
    field_type: Option<Type>,
    type_name: Option<String>,
    oneof_index: Option<i32>,
    proto3_optional: Option<bool>,
    default_value: Option<String>,
    options: Option<FieldOptions>,
}

#[derive(Debug, PartialEq)]
struct NormalizedEnum {
    values: Vec<NormalizedEnumValue>,
    reserved_ranges: Vec<NormalizedReservedRange>,
    reserved_names: Vec<String>,
    options: Option<EnumOptions>,
}

#[derive(Debug, PartialEq)]
struct NormalizedEnumValue {
    name: String,
    number: i32,
    options: Option<EnumValueOptions>,
}

fn normalize_message(message: &DescriptorProto) -> NormalizedMessage {
    NormalizedMessage {
        fields: message.field.iter().map(normalize_field).collect(),
        nested_messages: message
            .nested_type
            .iter()
            .map(|nested| NormalizedNestedMessage {
                name: nested.name.clone().expect("nested message name"),
                message: normalize_message(nested),
            })
            .collect(),
        nested_enums: message
            .enum_type
            .iter()
            .map(|nested| NormalizedNestedEnum {
                name: nested.name.clone().expect("nested enum name"),
                enumeration: normalize_enum(nested),
            })
            .collect(),
        oneofs: message.oneof_decl.iter().map(normalize_oneof).collect(),
        reserved_ranges: message
            .reserved_range
            .iter()
            .map(normalize_message_reserved_range)
            .collect(),
        reserved_names: message.reserved_name.clone(),
        options: message.options.clone(),
    }
}

fn normalize_field(field: &FieldDescriptorProto) -> NormalizedField {
    NormalizedField {
        name: field.name.clone().expect("field is missing name"),
        number: field.number(),
        label: field.label.and_then(|value| Label::try_from(value).ok()),
        field_type: field.r#type.and_then(|value| Type::try_from(value).ok()),
        type_name: field.type_name.as_deref().map(normalize_type_name),
        oneof_index: field.oneof_index,
        proto3_optional: field.proto3_optional,
        default_value: field.default_value.clone(),
        options: field.options.clone(),
    }
}

fn normalize_enum(enumeration: &EnumDescriptorProto) -> NormalizedEnum {
    NormalizedEnum {
        values: enumeration
            .value
            .iter()
            .map(|value| NormalizedEnumValue {
                name: normalize_enum_value_name(value.name.as_deref().expect("enum value name")),
                number: value.number(),
                options: value.options.clone(),
            })
            .collect(),
        reserved_ranges: enumeration
            .reserved_range
            .iter()
            .map(normalize_enum_reserved_range)
            .collect(),
        reserved_names: enumeration.reserved_name.clone(),
        options: enumeration.options.clone(),
    }
}

fn normalize_oneof(oneof: &OneofDescriptorProto) -> NormalizedOneof {
    NormalizedOneof {
        name: oneof.name.clone().expect("oneof name"),
        options: oneof.options.clone(),
    }
}

fn normalize_message_reserved_range(
    range: &descriptor_proto::ReservedRange,
) -> NormalizedReservedRange {
    NormalizedReservedRange {
        start: range.start(),
        end: range.end(),
    }
}

fn normalize_enum_reserved_range(
    range: &enum_descriptor_proto::EnumReservedRange,
) -> NormalizedReservedRange {
    NormalizedReservedRange {
        start: range.start(),
        end: range.end(),
    }
}

fn normalize_type_name(type_name: &str) -> String {
    let stripped = type_name
        .strip_prefix('.')
        .expect("descriptor type name should be fully-qualified");
    let stripped = stripped
        .strip_prefix(ORCHESTRATOR_PACKAGE)
        .or_else(|| stripped.strip_prefix(CONTROLPLANE_PACKAGE))
        .expect("descriptor type is outside the mirror packages");

    match stripped {
        ".ExperimentConfig" | ".ExperimentSpec" => ".ExperimentSpec".to_string(),
        other => other.to_string(),
    }
}

fn normalize_enum_value_name(name: &str) -> String {
    name.strip_prefix("PRUNE_ACTION_")
        .or_else(|| name.strip_prefix("ON_GOAL_"))
        .or_else(|| name.strip_prefix("POLICY_KIND_"))
        .or_else(|| name.strip_prefix("SCHED_MODE_"))
        .expect("enum value is outside the mirror enum closure")
        .to_string()
}
