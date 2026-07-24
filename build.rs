fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/winebridge.proto");
    tonic_prost_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"kebab-case\")]",
        )
        .compile_protos(&["proto/winebridge.proto"], &["proto/"])?;
    Ok(())
}
