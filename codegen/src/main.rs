//! This crate is used to generate the token streams for the macros in the `ctor` crate.
use std::fmt::Write;

pub mod macros;

/// Declare the macros in this crate. These macros will be available in the
/// codegen crate for testing, as well as in `TokenStream` form in the `ctor`
/// crate.
#[macro_export]
macro_rules! declare_macros {
    (
        $( $( #[doc = $doc:literal] )* macro_rules ! $name:ident $defn:tt )*
    ) => {
        $( #[allow(unused, unused_macro_rules, edition_2024_expr_fragment_specifier)] macro_rules! $name $defn )*

        $( pub(crate) use $name; )*

        /// Generate the token streams for the macros in this crate using `quote`.
        pub fn tokens() -> ::proc_macro2::TokenStream {
            quote::quote! {
                $(
                    #[allow(unused, unused_macro_rules, edition_2024_expr_fragment_specifier)] macro_rules! $name $defn
                    pub(crate) use $name;
                )*
            }
        }
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
    writeln!(w, "/// Generated macro token stream.")?;
    writeln!(w, "pub(crate) fn {name}() -> TokenStream {{")?;
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

    dump_tokens(&mut s, "ctor", macros::tokens())?;

    Ok(s)
}

macros::ctor_parse!(
    #[ctor]
    #[feature(__warn_on_missing_unsafe)]
    #[macro_path=macros]
    #[allow(deprecated)]
    fn foo_missing_unsafe() {}
);

macros::ctor_parse!(
    #[ctor]
    #[feature(__warn_on_missing_unsafe)]
    #[macro_path=macros]
    #[allow(deprecated)]
    unsafe fn foo_with_unsafe() {}
);

macros::ctor_parse!(
    #[ctor]
    #[feature(__warn_on_missing_unsafe)]
    #[macro_path=macros]
    /// Doc
    #[deprecated]
    #[allow(deprecated)]
    fn dtor_foo_missing_unsafe() {}
);

macros::ctor_parse!(
    #[ctor]
    #[feature(__warn_on_missing_unsafe)]
    #[macro_path=macros]
    /// Doc
    static STATIC_U8: u8 = { 1 };
);

macros::ctor_parse! {
    #[dtor]
    #[macro_path=macros]
    fn dtor() {

    }
}

macros::ctor_parse! {
    #[dtor]
    #[macro_path=macros]
    #[cfg(not(test))]
    fn dtor_with_cfg() {

    }
}

macros::ctor_parse! {
    #[dtor]
    #[macro_path=macros]
    #[cfg(test)]
    fn dtor_with_cfg() {

    }
}

mod module {
    use super::*;
    use std::collections::HashMap;

    macros::ctor_parse!(
        #[ctor]
        #[feature(__warn_on_missing_unsafe)]
        #[macro_path=macros]
        pub(crate) static STATIC_CTOR: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            _ = m.insert(0, "foo");
            _ = m.insert(1, "bar");
            _ = m.insert(2, "baz");
            m
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_to_date() {
        let contents = generate_code().expect("Failed to generate code");
        assert_eq!(
            std::fs::read_to_string("../ctor/src/gen.rs")
                .unwrap()
                .replace("\r\n", "\n"),
            contents
        );
    }
}
