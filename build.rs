use spirv_builder::{MetadataPrintout, SpirvBuilder, SpirvMetadata};
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/lib64");
    println!("cargo:rustc-link-lib=ze_loader");
    println!("cargo:rerun-if-changed=cpp/l0_wrapper.h");

    // TODO: parse_callbacks????
    let bindings = bindgen::Builder::default()
        .header("cpp/l0_wrapper.h")
        .generate()
        .expect("Unable to generate Level Zero bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write Level Zero bindings");

    // Build shader
    SpirvBuilder::new("shader", "spirv-unknown-spv1.5")
        .spirv_metadata(SpirvMetadata::NameVariables)
        .print_metadata(MetadataPrintout::Full)
        .build()
        .expect("Could not build shader");
}
