use link_section::{in_section, section};

#[section(data)]
pub static LINK_SECTION: link_section::Section;

#[in_section(LINK_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}

#[section(data)]
pub static TYPED_LINK_SECTION: link_section::TypedSection<u32>;

#[in_section(TYPED_LINK_SECTION)]
pub static LINKED_U32: u32 = 1;

pub fn main() {
}
