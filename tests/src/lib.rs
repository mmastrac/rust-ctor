// Prevent a spurious 'unused_imports' warning
#[allow(unused_imports)]
#[macro_use]
extern crate ctor;

#[cfg(test)]
mod test {
    use libc_print::*;
    use std::path::Path;
    use std::process::Command;
    use std::sync::atomic::{AtomicBool, Ordering};

    static INITED: AtomicBool = AtomicBool::new(false);
    static INITED_2: AtomicBool = AtomicBool::new(false);

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

    #[ctor]
    static INITED_3: u8 = 42;

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
        assert_eq!(*INITED_3, 42);
    }

    #[cfg(not(windows))]
    fn exe_extension() -> &'static str {
        ""
    }

    #[cfg(windows)]
    fn exe_extension() -> &'static str {
        ".exe"
    }

    #[cfg(target_feature = "crt-static")]
    fn crt_static() -> &'static str {
        "+crt-static"
    }

    #[cfg(not(target_feature = "crt-static"))]
    fn crt_static() -> &'static str {
        "-crt-static"
    }

    #[test]
    fn test_dylib() {
        let mut path = Path::new("..").canonicalize().unwrap();
        let exe = format!("dylib_load{}", exe_extension());
        for x in &["target", "debug", "examples", exe.as_str()] {
            path.push(x);
        }
        let mut cmd = Command::new(path.to_str().unwrap());

        // Move from tests -> root dir so we match the behaviour of running
        // --example
        let out = cmd.current_dir("..").output().unwrap();
        assert_eq!("", std::str::from_utf8(out.stdout.as_slice()).unwrap());

        let a = format!("+ ctor bin\n++ main start\n+++ ctor STATIC_INT\n+++ ctor lib ({})\n--- dtor lib\n-- main end\n- dtor bin\n", crt_static());
        let b = format!("+ ctor bin\n++ main start\n+++ ctor lib ({})\n+++ ctor STATIC_INT\n--- dtor lib\n-- main end\n- dtor bin\n", crt_static());
        let s = std::str::from_utf8(out.stderr.as_slice())
            .unwrap()
            .to_owned()
            .replace("\r\n", "\n");

        // There are two possible outcomes for stderr, depending on the order
        // that functions are called
        assert!(a == s || b == s);
    }
}
