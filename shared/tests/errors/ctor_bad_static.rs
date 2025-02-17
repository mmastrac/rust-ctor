shared::ctor_parse!(
    #[ctor]
    static U8: u8 = 42;
);

shared::ctor_parse!(
    #[ctor]
    static U8: u8 = unsafe { 42 };
);

shared::ctor_parse!(
    #[ctor]
    static U8: u8 = { 42 };
);

fn main() {}
