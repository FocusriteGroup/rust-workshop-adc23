use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=dsp");

    let path_to_dsp_library = build_dsp_library();

    link_to_dsp_library(path_to_dsp_library);
    link_to_libcpp();
    // generate_rust_bindings();
}

fn build_dsp_library() -> PathBuf {
    cmake::Config::new("dsp").build_target("dsp").build()
}

fn link_to_dsp_library(path_to_dsp_lib: PathBuf) {
    println!(
        "cargo:rustc-link-search=native={}",
        path_to_dsp_lib.join("build").display()
    );
    println!("cargo:rustc-link-lib=static=dsp");
}

fn link_to_libcpp() {
    match env::consts::OS {
        "macos" => println!("cargo:rustc-link-lib=c++"),
        "linux" => println!("cargo:rustc-link-lib=stdc++"),
        "windows" => {
            let msvcrt = match env::var("PROFILE").as_ref().map(String::as_str) {
                Ok("debug") => "msvcrtd",
                Ok("release") => "msvcrt",
                _ => panic!("unexpected profile"),
            };

            println!("cargo:rustc-link-lib=dylib={}", msvcrt);
        }
        _ => {}
    }
}

// fn generate_rust_bindings() {
//     let dsp_header = project_dir().join("dsp").join("include").join("dsp.h");
//
//     let bindings = bindgen::Builder::default()
//         .header(dsp_header.display().to_string())
//         .generate()
//         .unwrap();
//
//     let cargo_out_dir = env::var("OUT_DIR").unwrap();
//     let dsp_bindings_rs = env::var("DSP_BINDINGS").unwrap();
//     let path_to_bindings = PathBuf::from(cargo_out_dir).join(dsp_bindings_rs);
//     bindings.write_to_file(path_to_bindings).unwrap();
// }

fn project_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}
