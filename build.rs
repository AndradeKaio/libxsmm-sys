use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=xsmm");

    // If libxsmm is not installed system-wide, you may need:
    // println!("cargo:rustc-link-search=native=/path/to/libxsmm/lib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("libxsmm_dgemm")
        .allowlist_function("libxsmm_create_spgemm_csr_areg")
        .allowlist_type("libxsmm_gemm_shape")
        .allowlist_type("libxsmm_gemmfunction")
        .allowlist_type("libxsmm_bitfield")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
