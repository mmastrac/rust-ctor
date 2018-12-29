use ctor::*;

#[cfg(not(windows))]
pub fn shutdown_println(msg: &str) {
    unsafe {
        libc::write(
            2,
            std::mem::transmute(msg.as_ptr()),
            msg.len() as libc::size_t,
        );
        let newline = "\n";
        libc::write(2, std::mem::transmute(newline.as_ptr()), 1);
    }
}

#[cfg(windows)]
pub fn shutdown_println(msg: &str) {
    unsafe {
        libc::write(2, std::mem::transmute(msg.as_ptr()), msg.len() as u32);
        let newline = "\n";
        libc::write(2, std::mem::transmute(newline.as_ptr()), 1);
    }
}

#[ctor]
fn ctor() {
    shutdown_println("+++ ctor lib");
}

#[dtor]
fn dtor() {
    shutdown_println("--- dtor lib");
}
