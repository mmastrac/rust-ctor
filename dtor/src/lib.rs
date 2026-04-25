#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod features;
mod native;

#[cfg(feature = "export_native")]
pub use native::*;

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

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    use crate::features::*;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __dtor_parse {
        ( $($input:tt)* ) => {
            $crate::__perform!(
                ($($input)*),
                $crate::__chain[
                    $crate::__parse_item[$crate::dtor_parse],
                    $crate::__dtor_parse_impl,
                ]
            );
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __dtor_parse_impl {
        ( @entry next=$next:path[$next_args:tt], input=$input:tt ) => {
            $next ! ( $next_args, () );
        };
    }

    pub use __dtor_parse as dtor_parse;

    pub use crate::native::*;
}

__declare_features!(
    dtor: dtor_parse => dtor_impl;
    
    /// Enable support for the standard library. This is required for static
    /// ctor variables, but not for functions.
    std {
        feature: "std" = __include_std_feature;
    };
    /// Mark all ctor functions with `used(linker)`.
    used_linker {
        feature: "used_linker" = __include_used_linker_feature;
        attr: [(used(linker)) => (used_linker)];
    };
    /// Enable support for the proc-macro `#[ctor]` and `#[dtor]` attributes.
    proc_macro {
        feature: "proc_macro" = __include_proc_macro_feature;
    };
    /// Do not warn when a ctor or dtor is missing the `unsafe` keyword.
    no_warn_on_missing_unsafe {
        feature: "no_warn_on_missing_unsafe" = __include_no_warn_on_missing_unsafe_feature;
        attr: [(no_warn_on_missing_unsafe) => (no_warn_on_missing_unsafe)];
    };
    /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
    /// Specify a custom crate path for the `ctor` crate. Used when re-exporting the ctor macro.
    crate_path {
        attr: [(crate_path = $path:pat) => ($path)];
    };
    /// Make the ctor function anonymous.
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
);
