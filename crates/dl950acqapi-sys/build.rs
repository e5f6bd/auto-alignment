use std::path::PathBuf;

fn main() {
    let header_path = env!(
        "DL950ACQAPI_VC",
        "You must specifiy the path to the `vc` folder in `DL950ACQAPI`."
    );
    println!("cargo:rerun-if-env:changed=DL950ACQAPI_VC");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I")
        .clang_arg(header_path)
        .clang_args(["-x", "c++"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo::rustc-link-search={header_path}");
    println!("cargo::rustc-link-lib=DL950ACQAPI64");
}
