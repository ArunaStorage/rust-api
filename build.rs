extern crate tonic_build;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_services()?;
    Ok(())
}

fn compile_services() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("API/sciobjsdb/api/storage/services/v1/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "API/sciobjsdb/api/storage/services/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("API/sciobjsdb/api/notification/services/v1/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "API/sciobjsdb/api/notification/services/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    protos.push("proto/google/api/annotations.proto".to_string());
    protos.push("proto/google/api/http.proto".to_string());

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/sciobjectsdb/") // you can change the generated code's location
        .compile(
            &protos,
            &["proto/".to_string(), "API/".to_string()], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
}
