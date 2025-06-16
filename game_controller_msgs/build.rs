use std::{env, path::PathBuf};

use bindgen::{Builder, CargoCallbacks};

fn main() {
    let bindings = Builder::default()
        .header("headers/bindings.h")
        .allowlist_file("headers/(.*).h")
        .blocklist_type(".*")
        .fit_macro_constants(true)
        .layout_tests(false)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
