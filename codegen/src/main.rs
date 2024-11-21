use std::collections::HashMap;
use std::fmt::Write;

pub mod macros;

#[macro_export]
macro_rules! declare_macro {
    (macro_rules ! $name:ident $($defn:tt)* ) => {
        macro_rules! $name $($defn)*

        pub(crate) use $name;

        ::paste::paste!(pub fn [<$name _tokens>]() -> ::proc_macro2::TokenStream {
            quote::quote! {
                macro_rules! $name $($defn)*

                pub(crate) use $name;
            }
        });
    };
}

fn dump_tokens_recursive(
    indent: &str,
    w: &mut String,
    tokens: ::proc_macro2::TokenStream,
) -> Result<(), std::fmt::Error> {
    use ::proc_macro2::TokenTree::*;
    let tt = "T";

    if tokens.is_empty() {
        writeln!(w, "{indent}TokenStream::new()")?;
    } else {
        writeln!(w, "{indent}TokenStream::from_iter([")?;
        {
            let indent = format!("{indent}  ");
            for token in tokens {
                match token {
                    Ident(name) => {
                        let name = name.to_string();
                        writeln!(w, r#"{indent}{tt}::Ident(Ident::new({name:?}, c())),"#)?;
                    }
                    Punct(punct) => {
                        let spacing = punct.spacing();
                        let punct = punct.as_char();
                        writeln!(
                            w,
                            r#"{indent}{tt}::Punct(Punct::new({punct:?}, {spacing:?})),"#
                        )?;
                    }
                    Literal(literal) => {
                        let literal = literal.to_string();
                        if literal.starts_with('"') {
                            writeln!(w, "{indent}{tt}::Literal(Literal::string({literal})),")?;
                        } else {
                            writeln!(
                                w,
                                "{indent}{tt}::Literal(Literal::usize_unsuffixed({literal})),"
                            )?;
                        }
                    }
                    Group(group) => {
                        let delimiter = group.delimiter();
                        writeln!(w, "{indent}{tt}::Group(Group::new({delimiter:?}, ")?;
                        dump_tokens_recursive(&format!("{indent}  "), w, group.stream())?;
                        writeln!(w, "{indent})),")?;
                    }
                }
            }
        }
        writeln!(w, "{indent}])")?;
    }
    Ok(())
}

fn dump_tokens(
    w: &mut String,
    name: &str,
    tokens: ::proc_macro2::TokenStream,
) -> Result<(), std::fmt::Error> {
    writeln!(w, "pub fn {name}() -> TokenStream {{")?;
    dump_tokens_recursive("  ", w, tokens).unwrap();
    writeln!(w, "}}")?;
    writeln!(w)?;
    Ok(())
}

fn generate_code() -> Result<String, std::fmt::Error> {
    let mut s = String::with_capacity(16 * 1024);
    writeln!(&mut s, "// This file is generated. Do not edit.")?;
    writeln!(&mut s)?;

    writeln!(&mut s, "use proc_macro::*;")?;
    writeln!(&mut s, "use proc_macro::TokenTree as T;")?;
    writeln!(&mut s, "use proc_macro::Spacing::*;")?;
    writeln!(&mut s, "use proc_macro::Delimiter::*;")?;
    writeln!(&mut s, "use std::iter::FromIterator;")?;
    writeln!(&mut s)?;
    writeln!(&mut s, "fn c() -> Span {{ Span::call_site() }}")?;
    writeln!(&mut s)?;

    dump_tokens(&mut s, "ctor", macros::ctor_impl_tokens())?;
    dump_tokens(&mut s, "ctor_raw", macros::ctor_raw_tokens())?;

    Ok(s)
}

macros::ctor_impl!(
    fn
    macros=macros
    name=foo_impl
    used=used
    item={
        fn foo() {

        }
    }
);

macros::ctor_impl!(
    static
    macros=macros
    name=static_impl
    used=used
    item={
        pub static STATIC_U8: u8 = { 1 };
    }
);

macros::ctor_impl!(
    dtor
    macros=macros
    name=dtor_impl
    used=used
    item={
        fn bar() {

        }
    }
);

mod module {
    use super::*;
    macros::ctor_impl!(
        static
        macros=macros
        name=static_hash_impl
        used=used
        item={
            pub static STATIC_CTOR: HashMap<u32, &'static str> = {
                let mut m = HashMap::new();
                m.insert(0, "foo");
                m.insert(1, "bar");
                m.insert(2, "baz");
                m
            };
        }
    );
}

#[allow(unused)]
fn test_compiles() {
    eprintln!("{:?}", *STATIC_U8);
    eprintln!("{:?}", *module::STATIC_CTOR);
}

fn main() {
    let contents = generate_code().expect("Failed to generate code");
    std::fs::write("ctor/src/gen.rs", contents).expect("Failed to write code");
}
