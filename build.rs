use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    println!("cargo:rerun-if-changed=assets/config.cfg");

    let output_path = get_output_path();

    let _ = fs::create_dir(output_path.join("assets"));

    println!("{}", output_path.display());

    fs::copy(
        "assets/config.cfg",
        output_path.join("assets").join("config.cfg"),
    ).unwrap();
}
