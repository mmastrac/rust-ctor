#![allow(unexpected_cfgs)]

use ::features::*;

__declare_features!(
    small_macro: small_macro_parse => small_macro_impl;

    /// Enable support for the standard library. This is required for static
    /// ctor variables, but not for functions.
    std {
        feature: "std" = small_macro_include_std_feature;
    };
);


__declare_features!(
    my_macro: my_macro_parse => my_macro_impl;

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
    /// Enable support for the priority parameter.
    priority_enabled {
        feature: "priority" = __include_priority_feature;
    };
    /// Set the ctor priority to a given value.
    priority {
        attr: [(priority = $priority:literal) => ($priority)];
    };
    /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
    /// Place the initialization function pointer in a custom link section. This
    /// may cause the initialization function to fail to run or run earlier or
    /// later than other `ctor` functions.
    link_section {
        attr: [(link_section($section:literal)) => ($section)];
        default {
            target_vendor = "apple" => "__DATA,__mod_term_func,mod_term_funcs",
            target_vendor = "pc" => compile_error!("Link section dtor is not supported on PC"),
            target_os = "linux" => ".dtors",
            _ => compile_error!("Unsupported target vendor"),
        }
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


__test!(my_macro_parse[my_macro_parse => @extract (std)]:
    (std = std_value,) => (std = std_value,));
__test!(my_macro_parse[my_macro_parse => @extract (std)]:
    (std = std_value, anonymous = (),) => (std = std_value,));
__test!(my_macro_parse[my_macro_parse => @extract (std)]:
    (std = std_value, std = second_value,) => (std = second_value,));
__test!(my_macro_parse[my_macro_parse => @extract (std anonymous)]:
    (std = std_value, anonymous = (),) => (std = std_value, anonymous = (),));
__test!(my_macro_parse[my_macro_parse => @extract (std anonymous)]:
    (std = std_value,) => (std = std_value, anonymous = (),));


__test!(my_macro_parse[my_macro_parse => @meta]:
    (()) => ());
__test!(my_macro_parse[my_macro_parse => @meta]:
    ((unsafe)) => (unsafe = unsafe,));
__test!(my_macro_parse[my_macro_parse => @meta]:
    ((unsafe, priority = 1)) => (unsafe = unsafe, priority = 1,));
__test!(my_macro_parse[my_macro_parse => @meta]:
    ((priority = 1)) => (priority = 1,));


__test!(my_macro_parse[my_macro_parse => @crate]:
    () => (std = std,));


__test!(my_macro_parse[my_macro_parse => @self]:
    (#[my_macro]) => ((())()));
__test!(my_macro_parse[my_macro_parse => @self]:
    (#[my_macro(unsafe)]) => (((unsafe))()));
__test!(my_macro_parse[my_macro_parse => @self]:
    (#[my_macro(unsafe, priority = 1)]) => (((unsafe, priority = 1))()));
__test!(my_macro_parse[my_macro_parse => @self]:
    (#[my_macro(unsafe, priority = 1)] #[other_macro]) => (((unsafe, priority = 1))(#[other_macro])));
__test!(my_macro_parse[my_macro_parse => @self]:
    (#[other_macro]) => (()(#[other_macro])));
