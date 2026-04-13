#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod macros;

pub use macros::features;

#[doc(hidden)]
#[allow(unused)]
pub use macros::__support;

/// Marks a function as a library/executable destructor. This uses OS-specific
/// linker sections to call a specific function at termination time.
///
/// Multiple shutdown functions are supported, but the invocation order is not
/// guaranteed.
///
/// # Attribute parameters
///
///  - `crate_path = ::path::to::dtor::crate`: The path to the `dtor` crate
///    containing the support macros. If you re-export `dtor` items as part of
///    your crate, you can use this to redirect the macro's output to the
///    correct crate.
///  - `used(linker)`: (Advanced) Mark the function as being used in the link
///    phase.
///  - `link_section = "section"`: The section to place the dtor's code in.
///  - `anonymous`: Do not give the destructor a name in the generated code
///    (allows for multiple destructors with the same name).
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # use dtor::dtor;
/// # fn main() {}
///
/// #[dtor]
/// fn shutdown() {
///   /* ... */
/// }
/// ```
#[doc(inline)]
#[cfg(feature = "proc_macro")]
pub use dtor_proc_macro::dtor;

#[doc(hidden)]
#[cfg(feature = "proc_macro")]
pub use dtor_proc_macro::__dtor_from_ctor;

/// Declarative forms of the `#[dtor]` macro.
///
/// The declarative forms wrap and parse a proc_macro-like syntax like so, and
/// are identical in expansion to the undecorated procedural macros. The
/// declarative forms support the same attribute parameters as the procedural
/// macros.
///
/// ```rust
/// # #[cfg(any())] mod test { use dtor::*; use libc_print::*;
/// dtor::declarative::dtor! {
///   #[dtor]
///   fn foo() {
///     libc_println!("Goodbye, world!");
///   }
/// }
/// # }
///
/// // ... the above is identical to:
///
/// # #[cfg(any())] mod test_2 { use dtor::*; use libc_print::*;
/// #[dtor]
/// fn foo() {
///   libc_println!("Goodbye, world!");
/// }
/// # }
/// ```
pub mod declarative {
    #[doc(inline)]
    pub use crate::__support::dtor_parse as dtor;
}

#[cfg(feature = "export_native")]
pub use native::*;

mod native {
    /// Registers a raw function to be called at binary exit time.
    /// 
    /// Corresponds to `atexit` in C.
    #[allow(unused)]
        pub unsafe fn at_binary_exit(cb: extern "C" fn()) {
        _run_atexit(cb);
    }

    /// Registers a raw function to be called at library (libc calls this a DSO or
    /// "dynamic shared object") exit time.
    /// 
    /// Corresponds to `__cxa_atexit` in C, though the exit function argument is not
    /// available.
    #[cfg(feature = "cxa_atexit")]
    #[allow(unused)]
    pub unsafe fn at_library_exit(cb: extern "C" fn()) {
        _run_cxa_atexit(cb);
    }

    #[cfg(not(miri))]
    #[inline(always)]
    unsafe fn _run_atexit(cb: unsafe extern "C" fn()) {
        /*unsafe*/ extern "C" {
            fn atexit(cb: unsafe extern "C" fn());
        }
        unsafe {
            atexit(cb);
        }
    }

    // For platforms that have __cxa_atexit, we register the dtor as scoped to dso_handle
    #[cfg(all(not(miri), feature = "cxa_atexit"))]
    #[inline(always)]
    unsafe fn _run_cxa_atexit(cb: extern "C" fn()) {
        /*unsafe*/ extern "C" {
            static __dso_handle: *const u8;
            fn __cxa_atexit(cb: /*unsafe*/ extern "C" fn(_: *const u8), arg: *const u8, dso_handle: *const u8);
        }
        extern "C" fn exit_fn(fn_ptr: *const u8) {
            let f: fn() = unsafe { std::mem::transmute(fn_ptr) };
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

    #[cfg(miri)]
    #[inline(always)]
    unsafe fn _run_cxa_atexit(_cb: extern "C" fn()) {
        // no-op on miri
    }
}
