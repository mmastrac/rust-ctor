use ctor::ctor;

#[ctor]
static U8: u8 = 42;

#[ctor]
static U8: u8 = unsafe { 42 };

#[ctor]
static U8: u8 = { 42 };

#[ctor]
static U8: u8 = const { 42 };

fn main() {}
