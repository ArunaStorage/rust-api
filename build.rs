extern crate tonic_build;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos: Vec<String> = Vec::new();

    let entries = fs::read_dir("API/api/models/v1")?;

    for entry in entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "API/api/models/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path)
    }

    let entries = fs::read_dir("API/api/services/v1")?;

    for entry in entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "API/api/services/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path)
    }

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/sciobjectsdbapi") // you can change the generated code's location
        .compile(
            &protos,
            &["API/".to_string()], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
    
}
