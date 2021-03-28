use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let env_key = "SG_MAX_STACK_ELEMS";
    let env_val_def = "1024";

    println!("cargo:rerun-if-env-changed={}", env_key);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("consts.rs");

    // Soon this can be cargo-specified: https://stackoverflow.com/a/66509742
    let max_elems = match env::var(env_key) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("WARNING! Using default {} = {}", env_key, env_val_def);
            env_val_def.to_string()
        }
    };

    fs::write(
        &dest_path,
        format!("const MAX_ELEMS: usize = {};", max_elems),

    ).unwrap();
}