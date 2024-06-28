use std::path::PathBuf;

fn main() {
    let header_path = env!(
        "SPCM_C_HEADER",
        "You must specifiy the path to the `c_header` folder in `SPCM/Driver`."
    );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I")
        .clang_arg(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(r".*[/\\](regs|spcerr|spcm_drv)\.h")
        // .allowlist_item("(TYP|PCIBIT|SPCM|M2CMD|SPC|M2STAT)_.*")
        // .allowlist_item("ERRORTEXTLEN|COUPLING_(DC|AC)")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo::rustc-link-search={header_path}");
    println!("cargo::rustc-link-lib=spcm_win64_msvcpp");
}
