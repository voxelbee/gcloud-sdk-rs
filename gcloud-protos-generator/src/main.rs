use std::{fs, path::PathBuf};

mod gen;

fn main() {
    let proto_root = PathBuf::from("gcloud-protos-generator/proto/googleapis");
    let proto_includes = PathBuf::from("gcloud-protos-generator/protobuf");
    let mut protos = gen::find_proto(proto_root.clone());
    protos.extend(gen::find_proto(proto_includes.clone()));

    let out_dir = PathBuf::from("gcloud-sdk/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    let includes = [proto_root, proto_includes];

    let mut config = prost_build::Config::new();
    config
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_well_known_types()
        .extern_path(
            ".google.cloud.secretmanager.v1.SecretPayload",
            "crate::proto_ext::secretmanager::SecretPayload",
        )
        .extern_path(
            ".google.cloud.kms.v1.EncryptRequest",
            "crate::proto_ext::kms::EncryptRequest",
        )
        .extern_path(
            ".google.cloud.kms.v1.DecryptResponse",
            "crate::proto_ext::kms::DecryptResponse",
        )
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
        .extern_path(".google.protobuf.ListValue", "::prost_wkt_types::ListValue")
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .extern_path(".google.protobuf.Empty", "::prost_wkt_types::Empty")
        .extern_path(".google.protobuf.FieldMask", "::prost_wkt_types::FieldMask")
        .extern_path(".google.protobuf.NullValue", "::prost_wkt_types::NullValue")
        .extern_path(".google.protobuf.Api", "pb::Api")
        .extern_path(".google.protobuf.UInt32Value", "pb::UInt32Value")
        .extern_path(".google.protobuf.Enum", "pb::Enum")
        .extern_path(".google.protobuf.Type", "pb::Type")
        .extern_path(".google.protobuf.DescriptorProto", "pb::DescriptorProto")
        .extern_path(".google.protobuf.Int64Value", "pb::Int64Value")
        .extern_path(".google.protobuf.Int32Value", "pb::Int32Value")
        .extern_path(".google.protobuf.FloatValue", "pb::FloatValue")
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile_with_config(config, &gen::proto_path(&protos), &includes)
        .unwrap();

    let mut out_path = PathBuf::from("gcloud-sdk/src/google_apis.rs");
    let root = gen::from_protos(protos);
    fs::write(out_path.clone(), root.gen_code()).unwrap();

    let input_contents = fs::read_to_string(&out_path).unwrap();
    let syntax_tree = syn::parse_file(&input_contents).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(out_path.clone(), formatted).unwrap();

    out_path.pop();
}
