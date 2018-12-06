// Prevent a spurious 'unused_imports' warning
#[allow(unused_imports)]
#[macro_use]
extern crate ctor;

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

    static INITED: AtomicBool = ATOMIC_BOOL_INIT;
    static INITED_2: AtomicBool = ATOMIC_BOOL_INIT;

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

    /// Doc comment
    #[ctor]
    fn foo() {
        INITED.store(true, Ordering::SeqCst);
    }

    /// This ensures that we support more than of these
    #[ctor]
    fn foo_2() {
        INITED_2.store(true, Ordering::SeqCst);
    }

    #[dtor]
    fn shutdown() {
        // Using println or eprintln here will panic as Rust has shut down
        shutdown_println("We don't test shutdown, but if you see this message it worked!");
    }

    #[test]
    fn test_initialized() {
        // Test to see that the ctor ran
        assert_eq!(true, INITED.load(Ordering::SeqCst));
        assert_eq!(true, INITED_2.load(Ordering::SeqCst));
    }
}
