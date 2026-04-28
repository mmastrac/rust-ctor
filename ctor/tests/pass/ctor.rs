use ctor::ctor;

/// Doc
#[ctor]
/// Doc
unsafe fn foo() {
}

#[::ctor::ctor]
unsafe fn bar() {
}

#[ctor]
static STATIC_CTOR: std::collections::HashMap<u32, &'static str> = unsafe {
    let m = std::collections::HashMap::new();
    m
};

fn main() {
}
