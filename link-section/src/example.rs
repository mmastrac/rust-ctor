use link_section::{in_section, section};

#[section(code)]
pub static LINK_SECTION: link_section::Section;

#[in_section(LINK_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}

#[section(data)]
pub static TYPED_LINK_SECTION: link_section::TypedSection<u32>;

#[in_section(TYPED_LINK_SECTION)]
pub static LINKED_U32: u32 = 1;

#[in_section(TYPED_LINK_SECTION)]
pub static LINKED_U32_2: u32 = 2;

#[section(data)]
pub static FN_ARRAY: link_section::TypedSection<fn()>;

#[in_section(FN_ARRAY)]
pub fn linked_function() {
    eprintln!("linked_function");
}

#[in_section(FN_ARRAY)]
pub fn linked_function_2() {
    eprintln!("linked_function_2");
}

#[in_section(FN_ARRAY)]
pub static OTHER_FN: fn() = link_section_function;

#[section(data)]
pub static DEBUGGABLES: link_section::TypedSection<&'static (dyn ::core::fmt::Debug + Sync)>;

#[in_section(DEBUGGABLES)]
pub static DEBUGGABLE: &'static (dyn ::core::fmt::Debug + Sync) = &1;

#[in_section(DEBUGGABLES)]
pub static DEBUGGABLE_2: &'static (dyn ::core::fmt::Debug + Sync) = &2;

#[in_section(DEBUGGABLES)]
pub static DEBUGGABLE_FUNCTION: fn() = {
    fn debuggable_function() {
        eprintln!("debuggable_function");
    }
    &(debuggable_function as fn())
};

pub fn main() {
    eprintln!("LINK_SECTION: {:?}", LINK_SECTION);
    eprintln!("TYPED_LINK_SECTION: {:?}", TYPED_LINK_SECTION);
    eprintln!("CODE_SECTION: {:?}", FN_ARRAY);
    eprintln!("{:?}", FN_ARRAY.as_slice());
    for f in &FN_ARRAY {
        eprintln!("f: {:?}", f);
        f();
    }
    eprintln!("DEBUGGABLES: {:?}", DEBUGGABLES.as_slice());
}
