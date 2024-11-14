
fn main() {
    // Tell Cargo to re-run the build script if foo.c changes
    println!("cargo:rerun-if-changed=foo.c");

    // Compile the C code into an object file or static library
    cc::Build::new()
        .file("foo.c")
        .compile("libfoo.a");

    // Link the static library `libfoo.a` with the Rust project
    println!("cargo:rustc-link-lib=static=foo");

    // Optionally, specify the location of the library (if not in the default search path)
    // println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
}
