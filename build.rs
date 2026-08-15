use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_prost_build::configure()
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"kebab-case\")]",
        )
        .file_descriptor_set_path(out_dir.join("bottles_descriptor.bin"))
        .compile_protos(
            &[
                "proto/winebridge.proto",
                "proto/bottles/profiles/v1/profiles.proto",
                "proto/bottles/store/v1/store.proto",
                "proto/bottles/library/v1/library.proto",
                "proto/bottles/registry/v1/registry.proto",
                "proto/bottles/common/v1/common.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
