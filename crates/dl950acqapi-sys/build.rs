use std::path::PathBuf;

fn main() {
    let header_path = env!(
        "DL950ACQAPI_C_INCLUDE",
        "You must specifiy the include directory where `dl950acqapi-c.h` is located at as the environment variable `DL950ACQAPI_C_INCLUDE`."
    );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I")
        .clang_arg(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
