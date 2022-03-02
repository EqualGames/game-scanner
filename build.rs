extern crate prost_build;

use std::path::PathBuf;

fn main() {
    let blizzard_proto_path = PathBuf::from("src").join("blizzard").join("proto");
    println!("cargo:rerun-if-changed={}", blizzard_proto_path.to_string_lossy());

    println!("Compiling protos...");
    prost_build::Config::new()
        .out_dir(&blizzard_proto_path)
        .compile_protos(
            &[blizzard_proto_path.join("product_db.proto")],
            &[blizzard_proto_path.clone()],
        )
        .unwrap();
}
