#![doc = include_str!("../README.md")]

use std::iter::FromIterator;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod xx3;

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn in_section(attribute: TokenStream, item: TokenStream) -> TokenStream {
    generate("in_section", "link_section", attribute, item)
}

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn section(attribute: TokenStream, item: TokenStream) -> TokenStream {
    generate("section", "link_section", attribute, item)
}

fn decode_literal_string(name: &str, literal: Literal) -> String {
    let literal = literal.to_string();
    let Some(literal) = literal.strip_prefix('"') else {
        panic!("{}: Expected a literal string", name);
    };
    let Some(literal) = literal.strip_suffix('"') else {
        panic!("{}: Expected a literal string", name);
    };
    if !literal.contains('\\') {
        literal.to_string()
    } else {
        let mut output = String::with_capacity(literal.len());
        let mut iter = literal.chars();
        while let Some(c) = iter.next() {
            if c == '\\' {
                match iter.next() {
                    Some('n') => output.push('\n'),
                    Some('r') => output.push('\r'),
                    Some('t') => output.push('\t'),
                    Some('\\') => output.push('\\'),
                    Some('"') => output.push('"'),
                    Some('\'') => output.push('\''),
                    Some('0') => output.push('\0'),
                    Some('x') => {
                        let Some(c) = iter.next() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        let Some(c2) = iter.next() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        let Ok(c) = format!("{}{}", c, c2).parse::<u8>() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        output.push(char::from(c));
                    }
                    Some(_) => panic!("{}: Expected a valid escape sequence", name),
                    None => break,
                }
            } else {
                output.push(c);
            }
        }
        output
    }
}

fn decode_literal_strings(name: &str, item: TokenTree) -> String {
    let mut output = String::new();
    match item {
        TokenTree::Literal(literal) => {
            output.push_str(&decode_literal_string(name, literal));
        }
        TokenTree::Group(group) => {
            for token in group.stream().into_iter() {
                output.push_str(&decode_literal_strings(name, token));
            }
        }
        _ => {
            panic!("{}: Expected a literal string or group", name);
        }
    }
    output
}

/// If the input string is longer than the max length, replace the tail end of
/// the string with the hash of the string.
///
/// hash!(output input (prefix) hash_length max_length valid_section_chars)
#[allow(missing_docs)]
#[proc_macro]
pub fn hash(item: TokenStream) -> TokenStream {
    let mut item = item.into_iter();

    let Some(TokenTree::Group(group)) = item.next() else {
        panic!("output: Expected a group");
    };
    let group = group.stream();

    let Some(TokenTree::Ident(literal)) = item.next() else {
        panic!("input: Expected an identifier");
    };
    let literal = literal.to_string();

    let Some(prefix_group) = item.next() else {
        panic!("prefix: Expected a group");
    };
    let prefix = decode_literal_strings("prefix", prefix_group);

    let Some(suffix_group) = item.next() else {
        panic!("suffix: Expected a group");
    };
    let suffix = decode_literal_strings("suffix", suffix_group);

    let Some(TokenTree::Literal(hash_length)) = item.next() else {
        panic!("hash_length: Expected a literal integer");
    };
    let Ok(hash_length) = hash_length.to_string().parse::<usize>() else {
        panic!("hash_length: Expected a literal integer");
    };

    let Some(TokenTree::Literal(max_length)) = item.next() else {
        panic!("max_length: Expected a literal integer");
    };
    let Ok(max_length) = max_length.to_string().parse::<usize>() else {
        panic!("max_length: Expected a literal integer");
    };

    // Valid section chars: "..."
    let Some(TokenTree::Literal(valid_section_chars)) = item.next() else {
        panic!("valid_section_chars: Expected a literal string");
    };
    let valid_section_chars =
        decode_literal_string("valid_section_chars", valid_section_chars).into_bytes();

    // If the string is valid as-is, return it
    let output = if literal.len() < max_length
        && !literal
            .to_string()
            .contains(|c| c > '\u{007f}' || !valid_section_chars.contains(&(c as u8)))
    {
        format!("{prefix}{literal}{suffix}")
    } else {
        // Not valid, so we need to hash the string
        let mut output = String::with_capacity(max_length + prefix.len() + suffix.len());
        output.push_str(&prefix.to_string());
        let mut next = literal.chars();
        while output.len() < max_length - hash_length + prefix.len() {
            let Some(c) = next.next() else {
                break;
            };
            if c <= '\u{007f}' && valid_section_chars.contains(&(c as u8)) {
                output.push(c);
            }
        }

        let mut hash = xx3::xx3hash(&literal);
        while output.len() < max_length + prefix.len() {
            let c = valid_section_chars[hash as usize % valid_section_chars.len()];
            output.push(c as char);
            hash /= valid_section_chars.len() as u64;
        }
        output.push_str(&suffix);
        output
    };

    fn emit(tree: TokenStream, output: &str, found: &mut bool) -> TokenStream {
        if *found {
            return tree;
        }
        let mut stream = TokenStream::new();
        for input in tree.into_iter() {
            match input {
                _ if *found => stream.extend([input]),
                TokenTree::Ident(ident) if ident.to_string() == "__" => {
                    stream.extend([TokenTree::Literal(Literal::string(output))]);
                    *found = true;
                }
                TokenTree::Group(group) => stream.extend([TokenTree::Group(Group::new(
                    group.delimiter(),
                    emit(group.stream(), output, found),
                ))]),
                _ => stream.extend([input]),
            }
        }
        stream
    }

    let mut found = false;
    let stream = emit(group, &output, &mut found);
    if !found {
        panic!("output: Expected to find __");
    }
    TokenStream::from_iter([TokenTree::Group(Group::new(Delimiter::None, stream))])
}

#[allow(unknown_lints, tail_expr_drop_order)]
fn generate(
    macro_type: &str,
    macro_crate: &str,
    attribute: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let mut inner = TokenStream::new();

    // Search for crate_path in attributes
    let mut crate_path = None;
    let mut tokens = attribute.clone().into_iter().peekable();

    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = &token {
            if ident.to_string() == "crate_path" {
                // Look for =
                #[allow(unknown_lints, tail_expr_drop_order)]
                if let Some(TokenTree::Punct(punct)) = tokens.next() {
                    if punct.as_char() == '=' {
                        // Collect tokens until comma or end
                        let mut path = TokenStream::new();
                        while let Some(token) = tokens.peek() {
                            match token {
                                TokenTree::Punct(p) if p.as_char() == ',' => {
                                    tokens.next();
                                    break;
                                }
                                _ => {
                                    path.extend(std::iter::once(tokens.next().unwrap()));
                                }
                            }
                        }
                        crate_path = Some(path);
                        break;
                    }
                }
            }
        }
    }

    if attribute.is_empty() {
        // #[link_section]
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([TokenTree::Ident(Ident::new(
                    macro_type,
                    Span::call_site(),
                ))]),
            )),
        ]);
    } else {
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([
                    TokenTree::Ident(Ident::new(macro_type, Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, attribute)),
                ]),
            )),
        ]);
    }

    inner.extend(item);

    let mut invoke = crate_path.unwrap_or_else(|| {
        TokenStream::from_iter([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new(macro_crate, Span::call_site())),
        ])
    });

    invoke.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("__support", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(
            &format!("{macro_type}_parse"),
            Span::call_site(),
        )),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    invoke
}
