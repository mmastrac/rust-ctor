#![recursion_limit = "256"]

//! Procedural macro for defining global constructor/destructor functions.
//!
//! This provides module initialization/teardown functions for Rust (like
//! `__attribute__((constructor))` in C/C++) for Linux, OSX, and Windows via
//! the `#[ctor]` and `#[dtor]` macros.
//!
//! This library works and has been tested for Linux, OSX and Windows. This
//! library will also work as expected in both `bin` and `cdylib` outputs,
//! ie: the `ctor` and `dtor` will run at executable or library
//! startup/shutdown respectively.
//!
//! This library currently requires Rust > `1.31.0` at a minimum for the
//! procedural macro support.

// Code note:

// You might wonder why we don't use `__attribute__((destructor))`/etc for
// dtor. Unfortunately mingw doesn't appear to properly support section-based
// hooks for shutdown, ie:

// https://github.com/Alexpux/mingw-w64/blob/d0d7f784833bbb0b2d279310ddc6afb52fe47a46/mingw-w64-crt/crt/crtdll.c

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Marks a function or static variable as a library/executable constructor.
/// This uses OS-specific linker sections to call a specific function at
/// load time.
///
/// Multiple startup functions/statics are supported, but the invocation order is not
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
/// # use std::sync::atomic::{AtomicBool, Ordering};
/// static INITED: AtomicBool = AtomicBool::new(false);
///
/// #[ctor]
/// fn foo() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// ```
///
/// Initialize a `HashMap` at startup time:
///
/// ```rust
/// # extern crate ctor;
/// # use std::collections::HashMap;
/// # use ctor::*;
/// #[ctor]
/// static STATIC_CTOR: HashMap<u32, String> = {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
///
/// # pub fn main() {
/// #   assert_eq!(STATIC_CTOR.len(), 100);
/// #   assert_eq!(STATIC_CTOR[&20], "x*100=2000");
/// # }
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
/// #[cfg_attr(target_os = "linux", link_section = ".init_array")]
/// #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
/// #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// static FOO: extern fn() = {
///   #[cfg_attr(target_os = "linux", link_section = ".text.startup")]
///   extern fn foo() { /* ... */ };
///   foo
/// };
/// ```
#[proc_macro_attribute]
pub fn ctor(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse_macro_input!(function);
    if let syn::Item::Fn(function) = item {
        validate_item("ctor", &function);

        let syn::ItemFn {
            attrs,
            block,
            sig:
                syn::Signature {
                    ident,
                    unsafety,
                    constness,
                    abi,
                    ..
                },
            ..
        } = function;

        // Linux/ELF: https://www.exploit-db.com/papers/13234

        // Mac details: https://blog.timac.org/2016/0716-constructor-and-destructor-attributes/

        // Why .CRT$XCU on Windows? https://www.cnblogs.com/sunkang/archive/2011/05/24/2055635.html
        // 'I'=C init, 'C'=C++ init, 'P'=Pre-terminators and 'T'=Terminators

        let output = quote!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[cfg_attr(target_os = "linux", link_section = ".init_array")]
            #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            #(#attrs)*
            static #ident
            :
            #unsafety extern #abi #constness fn() =
            {
                #[cfg_attr(target_os = "linux", link_section = ".text.startup")]
                #unsafety extern #abi #constness fn #ident() #block;
                #ident
            }
            ;
        );

        // eprintln!("{}", output);

        output.into()
    } else if let syn::Item::Static(var) = item {
        let syn::ItemStatic {
            ident,
            mutability,
            expr,
            attrs,
            ty,
            vis,
            ..
        } = var;

        if let Some(_) = mutability {
            panic!("#[ctor]-annotated static objects must not be mutable");
        }

        if attrs.iter().any(|attr| {
            attr.path
                .segments
                .iter()
                .any(|segment| segment.ident == "no_mangle")
        }) {
            panic!("#[ctor]-annotated static objects do not support #[no_mangle]");
        }

        let ctor_ident =
            syn::parse_str::<syn::Ident>(format!("{}___rust_ctor___ctor", ident).as_ref())
                .expect("Unable to create identifier");
        let storage_ident =
            syn::parse_str::<syn::Ident>(format!("{}___rust_ctor___storage", ident).as_ref())
                .expect("Unable to create identifier");

        let output = quote!(
            // This is mutable, but only by this macro code!
            static mut #storage_ident: Option<#ty> = None;

            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #vis struct #ident<T> {
                _data: core::marker::PhantomData<T>
            }

            #(#attrs)*
            #vis static #ident: #ident<#ty> = #ident {
                _data: core::marker::PhantomData::<#ty>
            };

            impl core::ops::Deref for #ident<#ty> {
                type Target = #ty;
                fn deref(&self) -> &'static #ty {
                    unsafe {
                        #storage_ident.as_ref().unwrap()
                    }
                }
            }

            #[used]
            #[allow(non_upper_case_globals)]
            #[cfg_attr(target_os = "linux", link_section = ".init_array")]
            #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            static #ctor_ident
            :
            unsafe fn() = {
                #[cfg_attr(target_os = "linux", link_section = ".text.startup")]
                unsafe fn initer() {
                    #storage_ident = Some(#expr);
                }; initer }
            ;
        );

        // eprintln!("{}", output);

        output.into()
    } else {
        panic!("#[ctor] items must be functions or static globals");
    }
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
        attrs,
        block,
        sig:
            syn::Signature {
                ident,
                unsafety,
                constness,
                abi,
                ..
            },
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
            #[allow(non_upper_case_globals)]
            #[cfg_attr(target_os = "linux", link_section = ".init_array")]
            #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            #(#attrs)*
            static __dtor_export
            :
            unsafe extern #abi #constness fn() =
            {
                #[cfg_attr(target_os = "linux", link_section = ".text.exit")]
                #unsafety extern #abi #constness fn #ident() #block;
                #[cfg_attr(target_os = "linux", link_section = ".text.startup")]
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
    let syn::ItemFn { vis, sig, .. } = item;

    // Ensure that visibility modifier is not present
    match vis {
        syn::Visibility::Inherited => {}
        _ => panic!("#[{}] methods must not have visibility modifiers", typ),
    }

    // No parameters allowed
    if sig.inputs.len() > 0 {
        panic!("#[{}] methods may not have parameters", typ);
    }

    // No return type allowed
    match sig.output {
        syn::ReturnType::Default => {}
        _ => panic!("#[{}] methods must not have return types", typ),
    }
}
