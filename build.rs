extern crate prost_build;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=*.proto");

    println!("Compiling protos...");
    let blizzard_proto_path = PathBuf::from("src").join("blizzard").join("proto");
    prost_build::Config::new()
        .out_dir(&blizzard_proto_path)
        .compile_protos(
            &[blizzard_proto_path.join("product_db.proto")],
            &[blizzard_proto_path.clone()],
        )
        .unwrap();
}
