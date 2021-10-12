use std::env;
use std::fs;
use std::path::Path;

const ELEM_LIMIT: usize = 2048;

fn main() {
    let env_key_max_items = "SG_MAX_STACK_ELEMS"; // TODO: for v2.0, rename to "SG_MAX_STACK_ITEMS" (to reflect both set elements and map key/val pairs)
    let env_val_max_items_def = "2048";

    // Original paper's alpha, `a`, can be chosen in the range `0.5 <= a < 1.0`.
    // We choose 2/3, i.e. `a = 0.666...`, by default.
    // Please see CONFIG.md.
    let env_key_alpha_num = "SG_ALPHA_NUMERATOR";
    let env_val_alpha_num_def = "2.0";
    let env_key_alpha_denom = "SG_ALPHA_DENOMINATOR";
    let env_val_alpha_denom_def = "3.0";

    println!("cargo:rerun-if-env-changed={}", env_key_max_items);
    println!("cargo:rerun-if-env-changed={}", env_key_alpha_num);
    println!("cargo:rerun-if-env-changed={}", env_key_alpha_denom);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let consts_file_path = Path::new(&out_dir).join("consts.rs");

    let max_elems = match env::var(env_key_max_items) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "cargo:warning=Unset environment variable, using default: \'{}={}\'",
                env_key_max_items, env_val_max_items_def
            );
            env_val_max_items_def.to_string()
        }
    };

    assert!(
        max_elems.parse::<usize>().unwrap() <= ELEM_LIMIT,
        "Exceeded max item limit! Depending on OS and item size, this could risk stack overflow.
        Comment out this check in \'build.rs\' at your own risk."
    );

    let alpha_num = match env::var(env_key_alpha_num) {
        Ok(val) => val,
        Err(_) => env_val_alpha_num_def.to_string(),
    };

    let alpha_denom = match env::var(env_key_alpha_denom) {
        Ok(val) => val,
        Err(_) => env_val_alpha_denom_def.to_string(),
    };

    let alpha = alpha_num.parse::<f32>().unwrap() / alpha_denom.parse::<f32>().unwrap();
    assert!(
        0.5 <= alpha && alpha < 1.0,
        "Invalid alpha! Condition must hold: 0.5 <= (SG_ALPHA_NUMERATOR / SG_ALPHA_DENOMINATOR) < 1"
    );

    let consts_file_contents = format!(
        "const MAX_ELEMS: usize = {};\n
        const ALPHA_NUM: f32 = {};\n
        const ALPHA_DENOM: f32 = {};\n",
        max_elems, alpha_num, alpha_denom,
    );

    fs::write(&consts_file_path, &consts_file_contents).unwrap();
}
