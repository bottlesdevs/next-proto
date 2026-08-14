fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/winebridge.proto");
    tonic_prost_build::configure()
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"kebab-case\")]",
        )
        .compile_protos(
            &[
                "proto/winebridge.proto",
                "proto/bottles/profiles/v1/profiles.proto",
                "proto/bottles/common/v1/common.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
