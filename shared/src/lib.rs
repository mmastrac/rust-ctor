//! Shared macros and utilities for the `ctor` and `dtor` crates.

pub mod macros;
pub mod __support {
    pub use crate::macros::__support::*;
}

pub use ctor::declarative::ctor as ctor_parse;
pub use dtor::declarative::dtor as dtor_parse;

__declare_features!(
    my_macro: my_macro_parse;

    /// Enable support for the standard library. This is required for static
    /// ctor variables, but not for functions.
    std: {
        feature: "std" = __include_std_feature;
    };
    /// Mark all ctor functions with `used(linker)`.
    used_linker: {
        feature: "used_linker" = __include_used_linker_feature;
        attr: [used(linker)];
    };
    /// Enable support for the proc-macro `#[ctor]` and `#[dtor]` attributes.
    proc_macro: {
        feature: "proc_macro" = __include_proc_macro_feature;
    };
    /// Do not warn when a ctor or dtor is missing the `unsafe` keyword.
    no_warn_on_missing_unsafe: {
        feature: "no_warn_on_missing_unsafe" = __include_no_warn_on_missing_unsafe_feature;
        attr: [no_warn_on_missing_unsafe];
    };
    /// Enable support for the priority parameter.
    priority_enabled: {
        feature: "priority" = __include_priority_feature;
    };
    /// Set the ctor priority to a given value.
    priority: {
        attr: [priority = $priority:literal];
    };
    /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
    unsafe: {
        attr: [unsafe];
    };
    /// Place the initialization function pointer in a custom link section. This
    /// may cause the initialization function to fail to run or run earlier or
    /// later than other `ctor` functions.
    link_section: {
        attr: [link_section($section:literal)];
    };
    /// Specify a custom crate path for the `ctor` crate. Used when re-exporting the ctor macro.
    crate_path: {
        attr: [crate_path = $path:path];
    };
    /// Make the ctor function anonymous.
    anonymous: {
        attr: [anonymous];
    };
);
