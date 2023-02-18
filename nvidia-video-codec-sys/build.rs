extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder, output: &str) {
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*")
        // This is needed because bindgen puts the wrong format dynamic libraries
        .replace("::libloading", "libloading");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

// If env variable is not defined, try the default paths
fn find_dir(defaults: &[&'static str], env_key: &'static str) -> PathBuf {
    match env::var_os(env_key) {
        Some(val) => PathBuf::from(&val),
        _ => {
            for path in defaults {
                let path_buf = PathBuf::from(path);
                if path_buf.as_path().exists() {
                    return path_buf;
                }
            }
            panic!("\"{:?} do not exist\"", defaults);
        }
    }
}

fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=Cargo.lock");

    let nvc_include = find_dir(
        &["/opt/nvidia-video-codec/include", "/usr/include/nvidia-sdk"],
        "NVIDIA_VIDEO_CODEC_INCLUDE_PATH",
    );
    // println!("cargo:rerun-if-changed={}", nvc_include.to_string_lossy());
    let cuda = pkg_config::probe_library("cuda").expect("failed to find cuda from pkg-config");
    let cuda_include = &cuda.include_paths[0];
    // println!("cargo:rerun-if-changed={}", cuda_include.to_string_lossy());
    let cuda_link = &cuda.link_paths[0].to_string_lossy();
    // println!("cargo:rerun-if-changed={}", cuda_link);
    let cuda_header = cuda_include.join("cuda.h");
    // println!("cargo:rerun-if-changed={}", cuda_header.to_string_lossy());

    // TODO support windows
    println!("cargo:rustc-link-lib=dylib={}", "cuda");
    println!("cargo:rustc-link-lib=dylib={}", "nvcuvid");
    println!("cargo:rustc-link-lib=dylib={}", "nvidia-encode");
    println!("cargo:rustc-link-lib=dylib={}", "nppc");
    println!("cargo:rustc-link-lib=dylib={}", "nppicc");
    println!("cargo:rustc-link-search={}", cuda_link);

    let cuda_builder = common_builder()
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .dynamic_library_name("cuda")
        .header(cuda_header.to_string_lossy());

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(cuda_builder, "src/cuda.rs");

    let cuvid_builder = common_builder()
        .clang_arg(format!("-I{}", nvc_include.to_string_lossy()))
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .dynamic_library_name("nvcuvid")
        .header(nvc_include.join("nvcuvid.h").to_string_lossy());
    // println!(
    //     "cargo:rerun-if-changed={}",
    //     nvc_include.join("nvcuvid.h").to_string_lossy()
    // );

    format_write(cuvid_builder, "src/cuvid.rs");

    let encode_api_builder = common_builder()
        // .clang_arg(format!("-I{}", nvc_include.to_string_lossy()))
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .dynamic_library_name("nvidia_encode")
        .header(nvc_include.join("nvEncodeAPI.h").to_string_lossy());
    // println!(
    //     "cargo:rerun-if-changed={}",
    //     nvc_include.join("nvEncodeAPI.h").to_string_lossy()
    // );

    format_write(encode_api_builder, "src/encode_api.rs");

    let npp_builder = common_builder()
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .dynamic_library_name("nppc")
        .dynamic_library_name("nppicc")
        .header(cuda_include.join("nppcore.h").to_string_lossy())
        .header(
            cuda_include
                .join("nppi_color_conversion.h")
                .to_string_lossy(),
        );
    // println!(
    //     "cargo:rerun-if-changed={}",
    //     cuda_include.join("nppcore.h").to_string_lossy()
    // );
    // println!(
    //     "cargo:rerun-if-changed={}",
    //     cuda_include
    //         .join("nppi_color_conversion.h")
    //         .to_string_lossy()
    // );

    format_write(npp_builder, "src/npp.rs");
}
