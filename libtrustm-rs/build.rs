use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/home/mecha-3/mickledore-org/build-1/tmp/work/armv8a-poky-linux/libtrustm-rs/git-r0/recipe-sysroot/usr/lib/");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=trustm");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I../trustm_helper/include")
        .clang_arg("-I../trustm_lib")
        .clang_arg("-I../trustm_lib/optiga/include/optiga/ifx_i2c/")
        .clang_arg("-I../trustm_lib/optiga/include/optiga/")
        .clang_arg("-I../trustm_lib/optiga/include/optiga/pal/")
        .clang_arg("-I../trustm_lib/optiga/include/")
        .clang_arg("-I../trustm_lib/optiga/include/optiga/common/")
        .clang_arg("-I../trustm_lib/optiga/include/optiga/comms/")
        .clang_arg("-I/home/mecha-3/mickledore-org/build-1/tmp/work/armv8a-poky-linux/libtrustm-rs/git-r0/recipe-sysroot/usr/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
