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
///```rust,ignore
/// #[used]
/// #[cfg_attr(target_os = "linux", link_section = ".ctors")]
/// #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// static foo: extern fn() = {
///   extern fn foo() { ... };
///   foo
/// }
/// ```
#[proc_macro_attribute]
pub fn ctor(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let output: syn::Item = syn::parse(function).unwrap();

    if let syn::Item::Fn(syn::ItemFn {
        ident,
        vis,
        unsafety,
        constness,
        abi,
        block,
        decl,
        attrs,
        ..
    }) = output
    {
        // Ensure that visibility modifier is not present
        match vis {
            syn::Visibility::Inherited => {}
            _ => panic!("#[ctor] methods must not have visibility modifiers"),
        }

        // No parameters allowed
        if decl.inputs.len() > 0 {
            panic!("#[ctor] methods may not have parameters");
        }

        // No return type allowed
        match decl.output {
            syn::ReturnType::Default => {}
            _ => panic!("#[ctor] methods must not have return types"),
        }

        let output = quote!(
            #[used]
            #[cfg_attr(target_os = "linux", link_section = ".ctors")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
            #(#attrs)*
            static #ident
            :
            extern #unsafety #abi #constness fn() =
            { extern #unsafety #abi #constness fn #ident() #block; #ident }
            ;
        );

        // eprintln!("{}", output);

        output.into()
    } else {
        panic!("Expected a function");
    }
}
