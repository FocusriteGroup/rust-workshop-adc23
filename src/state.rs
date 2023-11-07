use std::{fs::read_to_string, path::PathBuf};

pub fn load(path_to_file: PathBuf) -> crate::DspArguments {
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    todo!("Read content from file");

    // https://docs.rs/serde_json/latest/serde_json/#parsing-json-as-strongly-typed-data-structures
    todo!("Deserialize contents of file into DspArguments")
}
