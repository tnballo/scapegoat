use std::env;
use std::fs;
use std::path::Path;

const ELEM_LIMIT: usize = 2048;

fn main() {
    let env_key = "SG_MAX_STACK_ELEMS";
    let env_val_def = "2048";

    println!("cargo:rerun-if-env-changed={}", env_key);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("consts.rs");

    let max_elems = match env::var(env_key) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "cargo:warning=Unset environment variable, using default: \'{}={}\'",
                env_key, env_val_def
            );
            env_val_def.to_string()
        }
    };

    assert!(max_elems.parse::<usize>().unwrap() <= ELEM_LIMIT);

    fs::write(
        &dest_path,
        format!("const MAX_ELEMS: usize = {};", max_elems),
    )
    .unwrap();
}
