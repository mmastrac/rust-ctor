// Prevent a spurious 'unused_imports' warning
#[allow(unused_imports)]
#[macro_use]
extern crate ctor;

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
    use libc_print::*;

    static INITED: AtomicBool = ATOMIC_BOOL_INIT;
    static INITED_2: AtomicBool = ATOMIC_BOOL_INIT;

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
        libc_eprintln!("We don't test shutdown, but if you see this message it worked!");
    }

    #[test]
    fn test_initialized() {
        // Test to see that the ctor ran
        assert_eq!(true, INITED.load(Ordering::SeqCst));
        assert_eq!(true, INITED_2.load(Ordering::SeqCst));
    }
}
