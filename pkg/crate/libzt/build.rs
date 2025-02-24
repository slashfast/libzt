extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=zt");

    let bindings;

    #[cfg(windows)]
    {
        bindings = bindgen::Builder::default()
            .header("src/include/ZeroTierSockets.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .size_t_is_usize(false)
            .generate()
            .expect("Unable to generate bindings");
    }

    #[cfg(not(windows))]
    {
        bindings = bindgen::Builder::default()
            .header("src/include/ZeroTierSockets.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libzt.rs"))
        .expect("Couldn't write bindings!");
}
