//! Procedural macro for defining module startup functions.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Marks a function as a library/executable constructor. This uses OS-specific
/// linker sections to call a specific function at load time.
///
/// Multiple startup functions are supported, but the invocation order is not
/// guaranteed.
///
/// # Examples
///
/// Print a startup message:
///
/// ```rust
/// # extern crate ctor;
/// # use ctor::*;
/// #[ctor]
/// fn foo() {
///   println!("Hello, world!");
/// }
///
/// # fn main() {
/// println!("main()");
/// # }
/// ```
///
/// Make changes to `static` variables:
///
/// ```rust
/// # extern crate ctor;
/// # use ctor::*;
/// # use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
/// static INITED: AtomicBool = ATOMIC_BOOL_INIT;
///
/// #[ctor]
/// fn foo() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// ```
///
/// # Details
///
/// The `#[ctor]` macro makes use of linker sections to ensure that a
/// function is run at startup time.
///
/// The above example translates into the following Rust code (approximately):
///
///```rust
/// #[used]
/// #[cfg_attr(target_os = "linux", link_section = ".ctors")]
/// #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// static foo: extern fn() = {
///   extern fn foo() { /* ... */ };
///   foo
/// };
/// ```
#[proc_macro_attribute]
pub fn ctor(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse_macro_input!(function);
    validate_item("ctor", &function);

    let syn::ItemFn {
        ident,
        unsafety,
        constness,
        abi,
        block,
        attrs,
        ..
    } = function;

    // Linux/ELF: https://www.exploit-db.com/papers/13234

    // Mac details: https://blog.timac.org/2016/0716-constructor-and-destructor-attributes/

    // Why .CRT$XCU on Windows? https://www.cnblogs.com/sunkang/archive/2011/05/24/2055635.html
    // 'I'=C init, 'C'=C++ init, 'P'=Pre-terminators and 'T'=Terminators

    let output = quote!(
        #[used]
        #[cfg_attr(target_os = "linux", link_section = ".ctors")]
        #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(windows, link_section = ".CRT$XCU")]
        #(#attrs)*
        static #ident
        :
        extern #unsafety #abi #constness fn() =
        { extern #unsafety #abi #constness fn #ident() #block; #ident }
        ;
    );

    // eprintln!("{}", output);

    output.into()
}

/// Marks a function as a library/executable destructor. This uses OS-specific
/// linker sections to call a specific function at termination time.
///
/// Multiple shutdown functions are supported, but the invocation order is not
/// guaranteed.
///
/// `sys_common::at_exit` is usually a better solution for shutdown handling, as
/// it allows you to use `stdout` in your handlers.
///
/// ```rust
/// # extern crate ctor;
/// # use ctor::*;
///
/// #[dtor]
/// fn shutdown() {
///   /* ... */
/// }
/// ```
#[proc_macro_attribute]
pub fn dtor(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse_macro_input!(function);
    validate_item("dtor", &function);

    let syn::ItemFn {
        ident,
        unsafety,
        constness,
        abi,
        block,
        attrs,
        ..
    } = function;

    let output = quote!(
        mod #ident {
            use super::*;

            // Avoid a dep on libc by linking directly
            extern "C" {
                fn atexit(cb: #unsafety extern fn());
            }

            #[used]
            #[cfg_attr(target_os = "linux", link_section = ".ctors")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            #(#attrs)*
            static __dtor_export
            :
            unsafe extern #abi #constness fn() =
            {
                #unsafety extern #abi #constness fn #ident() #block;
                unsafe extern fn __dtor_atexit() {
                    atexit(#ident);
                };
                __dtor_atexit
            };
        }
    );

    // eprintln!("{}", output);

    output.into()
}

fn validate_item(typ: &str, item: &syn::ItemFn) {
    let syn::ItemFn { vis, decl, .. } = item;

    // Ensure that visibility modifier is not present
    match vis {
        syn::Visibility::Inherited => {}
        _ => panic!("#[{}] methods must not have visibility modifiers", typ),
    }

    // No parameters allowed
    if decl.inputs.len() > 0 {
        panic!("#[{}] methods may not have parameters", typ);
    }

    // No return type allowed
    match decl.output {
        syn::ReturnType::Default => {}
        _ => panic!("#[{}] methods must not have return types", typ),
    }
}
