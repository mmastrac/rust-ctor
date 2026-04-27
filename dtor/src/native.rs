#![allow(unsafe_code, unused_unsafe, unknown_lints)]

/// Registers a raw function to be called at binary exit time.
///
/// Corresponds to `atexit` in C.
///
/// # Safety
///
/// Rust does not provide any safety guarantees about life-before-main or
/// life-after-main. Ordering of destructors is not guaranteed, nor that a
/// destructor will be called at all.
#[inline(always)]
pub unsafe fn at_binary_exit(cb: extern "C" fn()) {
    unsafe {
        _run_atexit(cb);
    }
}

/// Registers a raw function to be called at library (libc calls this a DSO
/// or "dynamic shared object") exit time.
///
/// Corresponds to `__cxa_atexit` in C, though the exit function argument is
/// not available.
///
/// Unsupported on Windows platforms.
/// 
/// # Safety
///
/// Rust does not provide any safety guarantees about life-before-main or
/// life-after-main. Ordering of destructors is not guaranteed, nor that a
/// destructor will be called at all.
#[cfg(all(not(windows), any(feature = "cxa_atexit", target_vendor = "apple")))]
#[inline(always)]
pub unsafe fn at_library_exit(cb: extern "C" fn()) {
    unsafe {
        _run_cxa_atexit(cb);
    }
}

/// Register a function to be called at libc exit time.
#[cfg(not(miri))]
#[inline(always)]
unsafe fn _run_atexit(cb: unsafe extern "C" fn()) {
    #[allow(missing_unsafe_on_extern)] // MSRV
    extern "C" {
        fn atexit(cb: unsafe extern "C" fn());
    }
    unsafe {
        atexit(cb);
    }
}

/// Register a function scoped to the current dynamic shared object.
#[cfg(all(not(miri), not(windows), any(feature = "cxa_atexit", target_vendor = "apple")))]
#[inline(always)]
unsafe fn _run_cxa_atexit(cb: extern "C" fn()) {
    #[allow(missing_unsafe_on_extern)] // MSRV
    extern "C" {
        static __dso_handle: *const u8;
        fn __cxa_atexit(
            cb: /*unsafe*/ extern "C" fn(_: *const u8),
            arg: *const u8,
            dso_handle: *const u8,
        );
    }
    extern "C" fn exit_fn(fn_ptr: *const u8) {
        let f: fn() = unsafe { ::core::mem::transmute(fn_ptr) };
        f()
    }
    unsafe {
        __cxa_atexit(exit_fn, cb as _, __dso_handle);
    }
}

#[cfg(miri)]
#[inline(always)]
unsafe fn _run_atexit(_cb: extern "C" fn()) {
    // no-op on miri
}

#[cfg(all(miri, not(windows)))]
#[inline(always)]
unsafe fn _run_cxa_atexit(_cb: extern "C" fn()) {
    // no-op on miri
}
