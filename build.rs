extern crate tonic_build;

use std::fs;

fn main() {
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("./src/protos/aruna/api/storage/services/v1/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/storage/services/v1/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/notification/services/v1/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/notification/services/v1/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/aruna/api/internal/v1/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/aruna/api/internal/v1/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    tonic_build
        ::configure()
        .build_server(true)
        .out_dir("./src/aruna")
        .compile(
            &protos,
            &[
                "./src/protos".to_string(),
                "./src/protos/aruna/api/google".to_string(),
                "./src/protos/aruna/api/protoc-gen-openapiv2".to_string(),
            ]
        )
        .unwrap();
}