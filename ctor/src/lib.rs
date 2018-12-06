extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

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
            pub static #ident
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
