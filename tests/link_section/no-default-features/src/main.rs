use link_section::declarative::{in_section, section};

section! {
    #[section]
    pub static SECT: link_section::TypedSection<fn()>;
}

in_section! {
    #[in_section(SECT)]
    pub fn _in_section_no_default_features() {
        println!("link-section-no-default-features:in-section");
    }
}

fn main() {
    for f in SECT.as_slice() {
        f();
    }
    println!("link-section-no-default-features:main");
}
