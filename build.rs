fn main() {
    if cfg!(windows) {
        use std::path::PathBuf;

        let mut lib_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        lib_path.push("lib");
        lib_path.push("windows");
        if cfg!(target_arch = "x86_64") {
            lib_path.push("x64");
        } else if cfg!(target_arch = "x86") {
            lib_path.push("x86");
        } else {
            panic!("Non-x86 Windows platforms are not supported.");
        }
        println!("cargo:rustc-link-search=all={}", lib_path.display());

        let mut out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        out_path.pop(); // /out/
        out_path.pop(); // /some-hashed-folder/
        out_path.pop(); // /build/
        out_path.push("SDL2.dll");
        lib_path.push("SDL2.dll");

        std::fs::copy(lib_path, out_path).ok();
    }
}
