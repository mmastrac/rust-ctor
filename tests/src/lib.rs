// Prevent a spurious 'unused_imports' warning
#[allow(unused_imports)]
#[macro_use]
extern crate ctor;
extern crate assert_cmd;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use libc_print::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

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

    #[test]
    fn test_dylib() {
        let mut cmd = Command::cargo_example("dylib_load").unwrap();

        // Move from tests -> root dir so we match the behaviour of running
        // --example
        let out = cmd.current_dir("..").unwrap();
        assert_eq!("", std::str::from_utf8(out.stdout.as_slice()).unwrap());
        assert_eq!(
            "+ ctor bin\n++ main start\n+++ ctor lib\n--- dtor lib\n-- main end\n- dtor bin\n",
            std::str::from_utf8(out.stderr.as_slice()).unwrap()
        );
    }
}
