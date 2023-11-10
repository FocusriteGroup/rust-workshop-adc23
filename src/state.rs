use std::{fs::read_to_string, path::PathBuf};

pub fn load(path_to_file: PathBuf) -> crate::DspArguments {
    let file_contents = read_to_string(&path_to_file).expect("failed to open file");
    serde_json::from_str(&file_contents).expect("failed to deserialize")
}
