shared::ctor_parse!(
    /// Doc
    #[ctor]
    /// Doc
    unsafe fn foo() {
    }
);

shared::ctor_parse!(
    #[ctor]
    static STATIC_CTOR: std::collections::HashMap<u32, &'static str> = unsafe {
        let m = std::collections::HashMap::new();
        m
    };
);

fn main() {
}
