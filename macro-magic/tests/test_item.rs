use ::macro_magic::*;

__declare_features!(
    my_macro: my_macro_parse;

    /// Enable support for the standard library. This is required for static
    /// ctor variables, but not for functions.
    std {
        feature: "std";
    };
    /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
    unsafe {
        attr: [(unsafe) => (unsafe)];
    };
    priority {
        attr: [(priority = $priority_value:literal) => ($priority_value)];
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
    (#[my_macro] unsafe fn foo() { /* ... */ }) => 
    ((#[my_macro]) (unsafe fn foo() { /* ... */ })));
__test!(__split_meta:
    (#[my_macro] #[other_macro] fn foo() { /* ... */ }) => 
    ((#[my_macro] #[other_macro]) (fn foo() { /* ... */ })));

__test!(__parse_item[my_macro_parse]:
(
    #[my_macro(unsafe, priority = 1)]
    fn foo() { /* ... */ }
) =>
(
    features = (std = std, unsafe = unsafe, priority = 1, anonymous = (),),
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
    features = (std = std, unsafe = unsafe, priority = (), anonymous = (),),
    meta = (#[other] #[doc]),
    item = (fn foo() { /* ... */ })
));
