use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build/build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut path = PathBuf::new();
    path.push(out_dir);

    let mut ancestors = path.ancestors();
    ancestors.next().unwrap();
    ancestors.next().unwrap();
    let target_dir = ancestors.next().map(|path| path.to_owned()).unwrap();

    println!(
        "cargo:rustc-env=FUZZING_ARTIFACTS_DIR={}",
        target_dir.display()
    );
}
