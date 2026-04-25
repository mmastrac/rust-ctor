use ::features::perform::*;
use ::features::*;

__declare_features!(
    my_macro: my_macro_parse => my_macro_impl;

    /// Enable support for the standard library. This is required for static
    /// ctor variables, but not for functions.
    std {
        feature: "std" = __include_std_feature;
    };
    /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
    priority {
        attr: [(priority = $priority:literal) => ($priority)];
    };
    /// Make the ctor function anonymous.
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
);


__test!(__split_meta:
    (#[my_macro] fn foo() { /* ... */ }) => 
    ((#[my_macro]) (fn foo() { /* ... */ })));
__test!(__split_meta:
    (#[my_macro] #[other_macro] fn foo() { /* ... */ }) => 
    ((#[my_macro] #[other_macro]) (fn foo() { /* ... */ })));


__test!(__parse_item[my_macro_parse]:
    (
        #[my_macro(unsafe, priority = 1)]
        fn foo() { /* ... */ }
    ) =>
    (
        features = (std = std, unsafe = unsafe, priority = 1,), 
        meta = (),
        item = (fn foo() { /* ... */ })
    ));

__test!(__parse_item[my_macro_parse]:
    (
        #[other]
        #[my_macro(unsafe)]
        #[doc]
        fn foo() { /* ... */ }
    ) =>
    (
        features = (std = std, unsafe = unsafe,), 
        meta = (#[other] #[doc]),
        item = (fn foo() { /* ... */ })
    ));
