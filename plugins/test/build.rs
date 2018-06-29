extern crate capnpc;

use std::path::Path;

fn main() {
    let schema_dir = Path::new("../../schema/");
    let schema_files = [
        "protocol.capnp",
    ];

    let mut command = capnpc::CompilerCommand::new();

    command.src_prefix(schema_dir);

    for file in schema_files.iter() {
        let file_path = schema_dir.join(file);
        command.file(&file_path);

        println!(
            "cargo:rerun-if-changed={}",
            file_path.to_str().expect("Invalid file path")
        );
    }

    command.run()
        .expect("Failed to compile schemas");
}
