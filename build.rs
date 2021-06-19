use cc::Build;
use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out_dir.display());

    File::create(out_dir.join("link.ld"))?.write_all(include_bytes!("link.ld"))?;

    Build::new()
        .file("entry.S")
        .target("aarch64-unknown-none-softfloat")
        .compiler("aarch64-none-elf-gcc")
        .compile("entry");

    println!("cargo:rerun-if-changed=entry.S");

    Ok(())
}
