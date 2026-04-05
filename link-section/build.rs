use std::{env, fs, path::PathBuf};

fn main() {
    if cfg!(target_vendor = "apple") || cfg!(target_vendor = "pc") {
        return;
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let script = out.join("link_section.ld");

    let ld = r#"
SECTIONS {
  .link_section_registry : ALIGN(8) {
    *(SORT_BY_NAME(.data.link_section.*))
  }
}
INSERT AFTER .data;
"#;

    fs::write(&script, ld).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-link-arg=-Wl,-T,{}", script.display());
}
