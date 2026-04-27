#![recursion_limit = "256"]
#![no_std]
#![doc = include_str!("../docs/GENERATED.md")]

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod parse;
mod priority;
pub mod statics;

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    #[cfg(feature = "dtor")]
    pub use dtor::declarative::dtor as dtor_parse;

    // Required for proc_macro.
    pub use crate::__ctor_parse as ctor_parse;
}

/// Declarative forms of the `#[ctor]` and `#[dtor]` macros.
///
/// The declarative forms wrap and parse a proc_macro-like syntax like so, and
/// are identical in expansion to the undecorated procedural macros. The
/// declarative forms support the same attribute parameters as the procedural
/// macros.
///
/// ```rust
/// # #[cfg(not(miri))] mod test { use ctor::*; use libc_print::*;
/// ctor::declarative::ctor! {
///   #[ctor]
///   fn foo() {
///     libc_println!("Hello, world!");
///   }
/// }
/// # }
///
/// // ... the above is identical to:
///
/// # #[cfg(not(miri))] mod test_2 { use ctor::*; use libc_print::*;
/// #[ctor]
/// fn foo() {
///   libc_println!("Hello, world!");
/// }
/// # }
/// ```
pub mod declarative {
    #[doc(inline)]
    pub use crate::__support::ctor_parse as ctor;
    #[doc(inline)]
    #[cfg(feature = "dtor")]
    pub use crate::__support::dtor_parse as dtor;
}

/// Marks a function or static variable as a library/executable constructor.
/// This uses OS-specific linker sections to call a specific function at load
/// time.
///
/// # Important notes
///
/// Rust does not make any guarantees about stdlib support for life-before or
/// life-after main. This means that the `ctor` crate may not work as expected
/// in some cases, such as when used in an `async` runtime or making use of
/// stdlib services.
///
/// Multiple startup functions/statics are supported, but the invocation order
/// is not guaranteed.
///
/// The `ctor` crate assumes it is available as a direct dependency, If you
/// re-export `ctor` items as part of your crate, you can use the `crate_path`
/// parameter to redirect the macro's output to the correct crate, or use the
/// [`declarative::ctor`] form.
///
/// # Attribute parameters
///
///  - `unsafe`: Removes the requirement to mark the function as `unsafe`
///    (recommended).
///  - `link_section = "section"`: The section to place the constructor in.
///  - `anonymous`: Do not give the constructor a name in the generated code
///    (allows for multiple constructors with the same name). Equivalent to
///    wrapping the constructor in an anonymous const (ie: `const _ = { ...
///    };`).
///  - `priority = N`: The priority of the constructor. Higher-N-priority
///    constructors are run last. `N` must be between 0 and 999 for ordering
///    guarantees (N >= 1000 ordering is platform-defined). Ordering with
///    respect to constructors without a priority is platform-defined.
///  - `crate_path = (Advanced) ::path::to::ctor::crate`: The path to the `ctor`
///    crate containing the support macros. If you re-export `ctor` items as
///    part of your crate, you can use this to redirect the macro's output to
///    the correct crate. Using the [`declarative::ctor`] form is preferred over
///    this parameter.
///  - `used(linker)`: (Advanced) Mark the function as being used in the link
///    phase.
///
/// # Examples
///
/// Print a startup message (using `libc_print` for safety):
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # #[cfg(not(miri))] mod test {
/// # use ctor::ctor;
/// use libc_print::std_name::println;
///
/// #[ctor(unsafe)]
/// fn foo() {
///   // Using libc_print which is safe in `#[ctor]`
///   println!("Hello, world!");
/// }
///
/// # fn main() {
/// println!("main()");
/// # }}
/// ```
///
/// Make changes to `static` variables:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// use ctor::*;
/// use std::sync::atomic::{AtomicBool, Ordering};
/// static INITED: AtomicBool = AtomicBool::new(false);
///
/// #[ctor(unsafe)]
/// fn set_inited() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// # }
/// ```
///
/// Initialize a `HashMap` at startup time:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// # use std::collections::HashMap;
/// # use ctor::*;
/// #[ctor(unsafe)]
/// pub static STATIC_CTOR: HashMap<u32, String> = {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// # pub fn main() {
/// #   assert_eq!(test::STATIC_CTOR.len(), 100);
/// #   assert_eq!(test::STATIC_CTOR[&20], "x*100=2000");
/// # }
/// ```
///
/// # Details
///
/// The `#[ctor]` macro makes use of linker sections to ensure that a function
/// is run at startup time.
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// # use ctor::*;
/// #[ctor(unsafe)]
/// fn my_init_fn() {
///   /* ... */
/// }
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately):
///
/// ```rust
/// # fn my_init_fn() {}
/// #[used]
/// #[cfg_attr(target_os = "linux", link_section = ".init_array")]
/// #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func,mod_init_funcs")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// /* ... other platforms elided ... */
/// static INIT_FN: extern fn() = {
///     extern fn init_fn() { my_init_fn(); };
///     init_fn
/// };
/// ```
///
/// For `static` items, the macro generates a `std::sync::OnceLock` that is
/// initialized at startup time. `#[ctor]` on `static` items requires the
/// default `std` feature.
///
/// ```rust
/// # mod test {
/// # use ctor::*;
/// # use std::collections::HashMap;
/// #[ctor]
/// static FOO: HashMap<u32, String> = unsafe {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately),
/// which eagerly initializes the `HashMap` inside a `OnceLock` at startup time:
///
/// ```rust
/// # mod test {
/// # use ctor::ctor;
/// # use std::collections::HashMap;
/// static FOO: FooStatic = FooStatic { value: ::std::sync::OnceLock::new() };
/// struct FooStatic {
///   value: ::std::sync::OnceLock<HashMap<u32, String>>,
/// }
///
/// impl ::core::ops::Deref for FooStatic {
///   type Target = HashMap<u32, String>;
///   fn deref(&self) -> &Self::Target {
///     self.value.get_or_init(|| unsafe {
///       let mut m = HashMap::new();
///       for i in 0..100 {
///         m.insert(i, format!("x*100={}", i*100));
///       }
///       m
///     })
///   }
/// }
///
/// #[ctor]
/// unsafe fn init_foo_ctor() {
///   _ = &*FOO;
/// }
/// # }
/// ```
#[doc(inline)]
#[cfg(feature = "proc_macro")]
pub use ctor_proc_macro::ctor;

/// Re-exported `#[dtor]` proc-macro from `dtor` crate.
///
/// See [`::dtor`] for more details.
#[doc(inline)]
#[cfg(all(feature = "dtor", feature = "proc_macro"))]
pub use dtor::__dtor_from_ctor as dtor; // note: this is the dtor proc macro that looks in ctor

__declare_features!(
    ctor: __ctor_features;

    /// Make the ctor function anonymous.
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
    /// Specify a custom crate path for the `ctor` crate. Used when re-exporting the ctor macro.
    crate_path {
        attr: [(crate_path = $path:pat) => (($path))];
        example: "crate_path = ::path::to::ctor::crate";
    };
    /// Place the destructor function pointer in a custom link section.
    link_section {
        attr: [(link_section = $section:literal) => ($section)];
        example: "link_section = \".ctors\"";
        default {
            // This is no longer supported by Apple
            (target_vendor = "apple") => "__DATA,__mod_init_func,mod_init_funcs",
            // Most LLVM/GCC targets can use .fini_array
            (any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku",
                target_family = "wasm"
            )) => ".init_array",
            // xtensa targets: .dtors
            (target_arch = "xtensa") => ".ctors",
            // Windows targets: CRT$XPU
            (all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc"))) => ".CRT$XCU",
            // ... except GNU
            (all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc")))) => ".ctors",
            _ => (compile_error!("Unsupported target for #[ctor]"))
        }
    };
    no_warn_on_missing_unsafe {
        /// crate
        /// Do not warn when a ctor or dtor is missing the `unsafe` keyword.
        feature: "no_warn_on_missing_unsafe";
        /// attr
        /// Marks a ctor/dtor as unsafe.
        attr: [(unsafe) => (no_warn_on_missing_unsafe)];
    };
    priority {
        attr: [(priority = $priority_value:tt) => ($priority_value)];
        validate: [(priority = $priority:literal), (priority = early), (priority = late)];
    };
    /// Enable support for the proc-macro `#[dtor]` attribute. The declarative
    /// form (`dtor!(...)`) is always available. It is recommended that crates
    /// re-exporting the `dtor` macro disable this feature and only use the
    /// declarative form.
    proc_macro {
        feature: "proc_macro";
    };
    /// Enable support for the standard library.
    std {
        feature: "std";
    };
    used_linker {
        /// crate
        /// Applies `used(linker)` to all `dtor`-generated functions. Requires nightly and `feature(used_with_arg)`.
        feature: "used_linker";
        /// attr
        /// Mark generated functions for this `dtor` as `used(linker)`. Requires nightly and `feature(used_with_arg)`.
        attr: [(used(linker)) => (used_linker)];
    };
);

#[cfg(doc)]
__generate_docs!(__ctor_features);
