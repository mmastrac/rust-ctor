use ctor::ctor;

/// Doc
#[ctor]
/// Doc
unsafe fn foo() {
}

#[::ctor::ctor]
unsafe fn bar() {
}

#[ctor(unsafe, priority = naked)]
unsafe fn naked() {
}

#[ctor(unsafe, priority = early)]
unsafe fn early() {
}

#[ctor(unsafe, priority = late)]
unsafe fn late() {
}

#[ctor]
static STATIC_CTOR: std::collections::HashMap<u32, &'static str> = unsafe {
    let m = std::collections::HashMap::new();
    m
};

fn main() {
}
