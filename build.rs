extern crate tonic_build;

use std::fs;

fn main() {

    // V2 
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/storage/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/storage/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/notification/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/notification/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/dataproxy/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/dataproxy/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/hooks/services/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/hooks/services/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/health/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/health/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("./src/protos/v2/aruna/api/storage/models/v2/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        let rel_path = format!(
            "{}{}",
            "./src/protos/v2/aruna/api/storage/models/v2/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }


    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::prost_wkt_types")
        .build_server(true)
        .out_dir("./src/v2/aruna")
        .compile(
            &protos,
            &[
                "./src/protos/v2".to_string(),
                "./src/protos/v2/aruna/api/google".to_string(),
                "./src/protos/v2/aruna/api/protoc-gen-openapiv2".to_string(),
            ],
        )
        .unwrap();

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::prost_wkt_types")
        .build_server(true)
        .build_transport(false)
        .out_dir("./src/v2/aruna_no_transport")
        .compile(
            &protos,
            &[
                "./src/protos/v2".to_string(),
                "./src/protos/v2/aruna/api/google".to_string(),
                "./src/protos/v2/aruna/api/protoc-gen-openapiv2".to_string(),
            ],
        )
        .unwrap();


    // V3
    let mut protos = Vec::new();
    let service_entries = fs::read_dir("./src/protos/v3/").unwrap();

    for entry in service_entries {
        let dir = entry.unwrap();
        if dir.file_name().to_str().unwrap().contains("timestamp") {
            continue
        }
        let rel_path = format!(
            "{}{}",
            "./src/protos/v3/",
            dir.file_name().to_str().unwrap()
        );
        protos.push(rel_path);
    }

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/v3")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .build_server(true)
        .compile(
            &protos,
            &[
                "./src/protos/v3",
            ]
        )
        .unwrap();

}
