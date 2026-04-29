use ctor::ctor;

#[ctor(unsafe, priority = naked)]
fn naked() {}
